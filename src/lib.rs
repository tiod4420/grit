use std::error::Error;

type GritError = Box<dyn Error>;
type Result<T> = std::result::Result<T, GritError>;

pub mod object;
pub mod repo;
