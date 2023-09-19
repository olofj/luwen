use kmdif::{PciError, PciOpenError};
use luwen_if::{chip::AxiError, error::PlatformError, ArcMsgError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LuwenError {
    #[error(transparent)]
    PlatformError(#[from] PlatformError),

    #[error(transparent)]
    PciOpenError(#[from] PciOpenError),

    #[error(transparent)]
    PciError(#[from] PciError),
}

impl From<ArcMsgError> for LuwenError {
    fn from(value: ArcMsgError) -> Self {
        LuwenError::PlatformError(value.into())
    }
}

impl From<AxiError> for LuwenError {
    fn from(value: AxiError) -> Self {
        LuwenError::PlatformError(value.into())
    }
}