use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use ini::Ini;

use sha1::{Digest, Sha1};

use crate::error::{GritError, Result};
use crate::object::{GitObject, GitObjectType};

const DEFAULT_BRANCH: &str = "master";
const DEFAULT_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.";

#[derive(Debug)]
pub struct GitRepository {
    gitdir: PathBuf,
    worktree: PathBuf,
    config: Ini,
}

impl GitRepository {
    pub fn create(worktree: impl AsRef<Path>) -> Result<Self> {
        let worktree = worktree.as_ref();

        let repo = Self::new_unchecked(worktree);

        if repo.worktree.exists() {
            if !repo.worktree.is_dir() {
                return Err(GritError::NotADirectory(repo.worktree));
            }

            if repo.gitdir.exists() {
                if !repo.gitdir.is_dir() {
                    return Err(GritError::NotADirectory(repo.gitdir));
                } else if repo.gitdir.read_dir()?.count() != 0 {
                    return Err(GritError::InvalidGitRepo(
                        "git directory is not empty".into(),
                    ));
                }
            }
        } else {
            fs::create_dir_all(&repo.worktree)?;
        }

        // .git/branches
        let branches = repo.dir(["branches"])?;
        fs::create_dir_all(branches)?;

        // .git/objects
        let objects = repo.dir(["objects"])?;
        fs::create_dir_all(objects)?;

        // .git/refs/heads
        let heads = repo.dir(["refs", "heads"])?;
        fs::create_dir_all(heads)?;

        // .git/refs/tags
        let tags = repo.dir(["refs", "tags"])?;
        fs::create_dir_all(tags)?;

        // .git/config
        let mut config = File::create(repo.path(["config"]))?;
        Self::default_config().write_to(&mut config)?;

        // .git/description
        let mut description = File::create(repo.path(["description"]))?;
        writeln!(description, "{}", DEFAULT_DESCRIPTION)?;

        // .git/HEAD
        let mut head = File::create(repo.path(["HEAD"]))?;
        writeln!(head, "ref: refs/heads/{}", DEFAULT_BRANCH)?;

        Ok(repo)
    }

    pub fn find(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let path = match path.as_os_str().is_empty() {
            true => path,
            false => Path::new("."),
        };

        for path in fs::canonicalize(path)?.ancestors() {
            if let Ok(repo) = Self::new(path) {
                return Ok(repo);
            }
        }

        Err(GritError::GitRepoNotFound)
    }

    pub fn read(&self, sha: impl AsRef<str>) -> Result<GitObject> {
        let sha = sha.as_ref();

        if !Self::is_hash(sha) {
            return Err(GritError::InvalidHash(sha.into()));
        }

        let path = self.file(["objects", &sha[..2], &sha[2..]])?;

        // Decompress file
        let file = File::open(path)?;
        let mut zlib = BufReader::new(ZlibDecoder::new(file));

        // Get header
        let mut header = Vec::new();
        let null_byte = zlib.read_until(0x00, &mut header)?;

        if Some(&0x00) != header.get(null_byte) {
            return Err(GritError::InvalidHeader("null byte not found".into()));
        }

        // Parse header
        let (obj, size) = std::str::from_utf8(&header[..null_byte - 1])
            .map_err(|_| GritError::InvalidHeader("invalid UTF-8".into()))?
            .split_once(' ')
            .ok_or(GritError::InvalidHeader("space not found".into()))?;

        let obj = obj.parse::<GitObjectType>()?;
        let size = size
            .parse::<usize>()
            .map_err(|_| GritError::InvalidHeader("invalid data length".into()))?;

        // Read remain of file
        let mut data = Vec::new();
        let data_sz = zlib.read_to_end(&mut data)?;

        if size == data_sz {
            Ok(GitObject::create(obj, data))
        } else {
            Err(GritError::InvalidHeader(format!(
                "header size {} != data size {}",
                size, data_sz
            )))
        }
    }

    pub fn write(&self, object: &GitObject) -> Result<()> {
        // Serialize data and make header
        let header = object.header();
        let data = object.serialize();

        // Compute hash
        let mut sha = Sha1::new();
        sha.update(&header);
        sha.update(&data);
        let sha = format!("{:x}", sha.finalize());

        // Write compressed data to file
        let path = self.file(["objects", &sha[..2], &sha[2..]])?;

        // No need to write the file if already stored
        if !path.exists() {
            // Path has a parent by construct
            let parent = path.parent().unwrap();

            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }

            let mut zlib = ZlibEncoder::new(Vec::new(), Compression::default());
            zlib.write_all(&header)?;
            zlib.write_all(&data)?;
            let compressed = zlib.finish()?;

            let mut file = File::create(path)?;
            file.write_all(&compressed)?;
        }

        Ok(())
    }

    fn new(worktree: impl AsRef<Path>) -> Result<Self> {
        let worktree = worktree.as_ref().to_owned();
        let gitdir = worktree.join(".git");
        let config = gitdir.join("config");

        if !gitdir.is_dir() {
            return Err(GritError::GitRepoNotFound);
        }

        let config = Ini::load_from_file(config).map_err(|_| GritError::MissingConfigFile)?;

        let version = config
            .get_from(Some("core"), "repositoryformatversion")
            .and_then(|val| val.parse::<i32>().ok())
            .unwrap_or(-1);

        if version != 0 {
            return Err(GritError::InvalidGitRepo(format!(
                "unsupported repositoryformatversion {}",
                version
            )));
        }

        Ok(Self {
            worktree,
            gitdir,
            config,
        })
    }

    fn new_unchecked(worktree: impl AsRef<Path>) -> Self {
        let worktree = worktree.as_ref().to_owned();
        let gitdir = worktree.join(".git");
        let config = gitdir.join("config");

        let config = Ini::load_from_file(config).unwrap_or_default();

        Self {
            worktree,
            gitdir,
            config,
        }
    }

    fn path<P: AsRef<Path>>(&self, paths: impl AsRef<[P]>) -> PathBuf {
        paths
            .as_ref()
            .iter()
            .map(AsRef::as_ref)
            .fold(self.gitdir.clone(), |acc, path| acc.join(path))
    }

    fn file<P: AsRef<Path>>(&self, paths: impl AsRef<[P]>) -> Result<PathBuf> {
        let path = self.path(paths);

        if !path.exists() || path.is_file() {
            Ok(path)
        } else {
            Err(GritError::NotAFile(path))
        }
    }

    fn dir<P: AsRef<Path>>(&self, paths: impl AsRef<[P]>) -> Result<PathBuf> {
        let path = self.path(paths);

        if !path.exists() || path.is_dir() {
            Ok(path)
        } else {
            Err(GritError::NotADirectory(path))
        }
    }

    fn default_config() -> Ini {
        let mut config = Ini::new();

        config
            .with_section(Some("core"))
            .set("bare", "false")
            .set("repositoryformatversion", "0")
            .set("filemode", "false");

        config
    }

    fn is_hash(hash: impl AsRef<str>) -> bool {
        let hash = hash.as_ref();
        hash.len() == Sha1::output_size() && hash.chars().all(|c| c.is_ascii_hexdigit())
    }
}
