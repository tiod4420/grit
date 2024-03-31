use sha1::{Digest, Sha1};

pub fn hash(header: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> String {
    let mut sha = Sha1::new();

    sha.update(header);
    sha.update(data);

    format!("{:x}", sha.finalize())
}

pub fn is_hash(hash: impl AsRef<str>) -> bool {
    let hash = hash.as_ref();
    hash.len() == Sha1::output_size() && hash.chars().all(|c| c.is_ascii_hexdigit())
}
