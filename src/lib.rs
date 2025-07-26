
#![no_std]

#[cfg(test)]
extern crate alloc;

pub mod error;
pub mod iqs231x;

pub use error::Iqs231xError;
pub use iqs231x::Iqs231xDriver;