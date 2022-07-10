//! Peripheral access API for EFM32JG1B microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32jg-pacs)
//!
//! This crate supports all EFM32JG1B devices; for the complete list please see:
//! [efm32jg1b](https://github.com/efm32-rs/efm32jg-pacs/pacs/efm32jg1b)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32jg1b100")]
pub mod efm32jg1b100;

#[cfg(feature = "efm32jg1b200")]
pub mod efm32jg1b200;
