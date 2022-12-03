use std::error::Error;

pub type BoxError = Box<dyn Error>;

pub type Result<T> = std::result::Result<T, BoxError>;
