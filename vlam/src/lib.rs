#![no_std]
#![no_main]

pub mod context;
pub mod array;
pub(crate) mod arch;
mod utils;

pub use context::VLACtx;
pub use array::VLArray;

/// attribute macro used to safely set up the vla context on a function
pub use vlam_macro::vlam;