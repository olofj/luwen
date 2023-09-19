use axi::AxiError;
use luwen_core::Arch;
use thiserror::Error;

pub mod axi;
mod common;
mod grayskull;
pub mod remote;
mod wormhole;

use common::Chip;
use kmdif::{PciError, PciOpenError};

pub use common::ArcMsg;
pub use grayskull::Grayskull;
pub use kmdif::DmaConfig;
pub use remote::{detect::run_on_all_chips, EthCoord};
pub use wormhole::Wormhole;

#[derive(Error, Debug)]
pub enum TTError {
    #[error(transparent)]
    PciOpenError(#[from] PciOpenError),

    #[error(transparent)]
    PciError(#[from] PciError),

    #[error(transparent)]
    AxiError(#[from] AxiError),

    #[error("Chip arch mismatch: expected {expected:?}, got {actual:?}")]
    ArchMismatch { expected: Arch, actual: Arch },

    #[error("Unkown chip arch {0}")]
    UnknownArch(u16),
}

pub enum AllChips {
    Wormhole(Wormhole),
    Grayskull(Grayskull),
}

pub fn create_chip(device_id: usize) -> Result<AllChips, TTError> {
    let chip = Chip::create(device_id)?;

    match chip.arch() {
        Arch::Grayskull => Ok(AllChips::Grayskull(Grayskull::new(chip)?)),
        Arch::Wormhole => Ok(AllChips::Wormhole(Wormhole::new(chip)?)),
        Arch::Unknown(v) => Err(TTError::UnknownArch(v)),
    }
}

pub fn scan() -> Result<Vec<AllChips>, TTError> {
    let mut output = Vec::new();
    for id in kmdif::PciDevice::scan() {
        output.push(create_chip(id)?);
    }

    Ok(output)
}
