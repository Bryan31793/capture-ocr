use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Write to socket failed")]
    OcrRequestWrite(#[from] std::io::Error),

    #[error("Serialization of request failed")]
    OcrRequestSerialization(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Reading operation from socket failed")]
    OcrResponseRead(#[from] std::io::Error),

    #[error("input structure doesn't match structure expected")]
    OcrResponseDeserealization(#[from] serde_json::Error),
}