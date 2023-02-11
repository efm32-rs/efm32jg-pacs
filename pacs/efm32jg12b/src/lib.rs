//! Peripheral access API for EFM32JG12B microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32jg-pacs)
//!
//! This crate supports all EFM32JG12B devices; for the complete list please see:
//! [efm32jg12b](https://github.com/efm32-rs/efm32jg-pacs/pacs/efm32jg12b)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32jg12b500")]
pub mod efm32jg12b500;
