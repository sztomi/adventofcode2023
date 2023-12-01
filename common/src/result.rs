use std::error::Error;

pub type DynError = Box<dyn Error>;
pub type Result<T> = std::result::Result<T, DynError>;
