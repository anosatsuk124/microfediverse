use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum RustError {
    #[error("Server Error: {:?}", 0)]
    HyperError(#[from] HyperError),
}

#[derive(Debug, Error)]
#[error("{:?}", 0)]
pub struct HyperError(#[from] hyper::Error);

impl Serialize for HyperError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("HyperError", &format!("{:?}", self.0))
    }
}
