//#![deny(missing_docs)]
#![doc(html_root_url = "http://docs.rs/nvapi/0.2.0")]

pub use nvapi_sys as sys;

mod clock;
mod gpu;
#[cfg(feature = "i2c")]
mod i2c_impl;
mod info;
mod pstate;
mod thermal;
mod types;

pub use clock::*;
pub use gpu::*;
#[cfg(feature = "i2c")]
pub use i2c_impl::*;
pub use info::*;
pub use pstate::*;
pub use thermal::*;
pub use types::*;

pub use sys::{Result, Status};
