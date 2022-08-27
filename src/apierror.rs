use std::fmt;


#[derive(Debug)]
pub enum ApiError {
    TargetNotFound,
    NoUpdateRequired,
    InvalidData,
    DatabaseInternalError,
    AlreadyExists,
    Error(String)
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Error(err) => {
                write!(f, "{}", err)
            },
            _ => write!(f, "{:?}", self)
        }
    }
}