use thiserror::Error as ThisError;

#[derive(ThisError, Clone, Debug, PartialEq)]

pub enum Error {
    /// request error
    #[error("Http Request Error")]
    RequestError,

    #[error("Deserialize Error")]
    DeserializeError,

    #[error("Invalid request url")]
    InvalidReqUrl,

    #[error("Endpoint url unreachable")]
    UnreachableEndpoint,
}