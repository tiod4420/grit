use std::fmt;
use std::str;

use crate::error::GritError;
use crate::utils;

#[derive(Clone, Copy, Debug)]
pub enum GitObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}

impl fmt::Display for GitObjectType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Blob => write!(fmt, "blob"),
            Self::Commit => write!(fmt, "commit"),
            Self::Tag => write!(fmt, "tag"),
            Self::Tree => write!(fmt, "tree"),
        }
    }
}

impl str::FromStr for GitObjectType {
    type Err = GritError;

    fn from_str(object: &str) -> std::result::Result<Self, Self::Err> {
        match object.to_ascii_lowercase().as_str() {
            "blob" => Ok(Self::Blob),
            "commit" => Ok(Self::Commit),
            "tag" => Ok(Self::Tag),
            "tree" => Ok(Self::Tree),
            object => Err(Self::Err::InvalidObjectType(object.into())),
        }
    }
}

#[derive(Debug)]
pub struct GitObject {
    obj: GitObjectType,
    data: Vec<u8>,
}

impl GitObject {
    pub fn create(obj: GitObjectType, data: impl AsRef<[u8]>) -> Self {
        let data = data.as_ref();

        let data = match obj {
            GitObjectType::Blob => data.into(),
            _ => todo!(),
        };

        Self { obj, data }
    }

    pub fn header(&self) -> Vec<u8> {
        let len = match self.obj {
            GitObjectType::Blob => self.data.len(),
            _ => todo!(),
        };

        format!("{} {}\x00", self.obj, len).into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self.obj {
            GitObjectType::Blob => self.data.clone(),
            _ => todo!(),
        }
    }

    pub fn hash(&self) -> String {
        utils::hash(self.header(), self.serialize())
    }
}
