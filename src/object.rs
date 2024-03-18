use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

use crate::repo::GitRepository;
use crate::{GritError, Result};

#[derive(Debug)]
pub enum GitObject {
    Blob(Vec<u8>),
    Commit,
    Tag,
    Tree,
}

impl GitObject {
    pub fn read(repo: &GitRepository, sha: impl AsRef<str>) -> Result<Self> {
        let sha = sha.as_ref();

        if sha.len() != Sha1::output_size() {
            return Err(format!("Invalid Sha1 hash {}", sha).into());
        }

        let path = repo.file(["objects", &sha[..2], &sha[2..]])?;

        // Decompress file
        let file = File::open(path)?;
        let mut zlib = BufReader::new(ZlibDecoder::new(file));

        let mut header = Vec::new();
        let null_byte = zlib.read_until(0x00, &mut header)?;

        if Some(&0x00) != header.get(null_byte) {
            return Err("Invalid Object header, null byte not found".into());
        }

        let (obj_type, size) = std::str::from_utf8(&header[..null_byte - 1])?
            .split_once(' ')
            .ok_or::<GritError>("Invalid Object header, space not found".into())?;

        let size = size.parse::<usize>()?;

        // Read remain of file
        let mut data = Vec::new();
        let data_sz = zlib.read_to_end(&mut data)?;

        if size == data_sz {
            Self::new(obj_type, data)
        } else {
            Err(format!("Size in header {} != data size {}", size, data_sz).into())
        }
    }

    pub fn serialize(&self) -> Result<&[u8]> {
        match self {
            Self::Blob(data) => Ok(&data),
            _ => todo!(),
        }
    }

    fn new(obj_type: &str, data: Vec<u8>) -> Result<Self> {
        match obj_type {
            "blob" => Ok(Self::Blob(data)),
            "commit" | "tag" | "tree" => todo!(),
            t => Err(format!("Unknown Git Object type {}", t))?,
        }
    }

    fn obj_type(&self) -> &str {
        match self {
            Self::Blob(_) => "blob",
            Self::Commit => "commit",
            Self::Tag => "tag",
            Self::Tree => "tree",
        }
    }

    fn write(&self, repo: &GitRepository) -> Result<()> {
        // Serialize data and make header
        let data = self.serialize()?;
        let header = format!("{} {}\x00", self.obj_type(), data.len());

        // Compute hash
        let mut sha = Sha1::new();
        sha.update(header.as_bytes());
        sha.update(data);
        let sha = format!("{:x}", sha.finalize());

        // Write compressed data to file
        let path = repo.file(["objects", &sha[..2], &sha[2..]])?;

        // No need to write the file if already stored
        if !path.exists() {
            // Path has a parent by construct
            let parent = path.parent().unwrap();

            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }

            let mut zlib = ZlibEncoder::new(Vec::new(), Compression::default());
            zlib.write_all(header.as_bytes())?;
            zlib.write_all(data)?;
            let compressed = zlib.finish()?;

            let mut file = File::create(path)?;
            file.write_all(&compressed)?;
        }

        Ok(())
    }
}
