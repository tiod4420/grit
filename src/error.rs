use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum GritError {
    GitRepoNotFound,
    InvalidGitRepo(String),
    InvalidHash(String),
    InvalidHeader(String),
    InvalidObjectType(String),
    IOError(io::Error),
    MissingConfigFile,
    NotADirectory(PathBuf),
    NotAFile(PathBuf),
}

impl fmt::Display for GritError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GitRepoNotFound => write!(
                fmt,
                "not a git repository (or any parent up to mount point /)"
            ),
            Self::InvalidGitRepo(msg) => write!(fmt, "Invalid git repository: {}", msg),
            Self::InvalidHash(hash) => write!(fmt, "Invalid object name: {}", hash),
            Self::InvalidHeader(msg) => write!(fmt, "Invalid object header: {}", msg),
            Self::InvalidObjectType(msg) => write!(fmt, "Invalid object type: {}", msg),
            Self::IOError(err) => err.fmt(fmt),
            Self::MissingConfigFile => write!(fmt, "Missing configuration file"),
            Self::NotADirectory(path) => write!(fmt, "{} is not a directory", path.display()),
            Self::NotAFile(path) => write!(fmt, "{} is not a regular file", path.display()),
        }
    }
}

impl From<io::Error> for GritError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl Error for GritError {}

pub type Result<T> = std::result::Result<T, GritError>;
