//! Darwinia bridge primitives
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::assign_op_pattern)]
#![warn(missing_docs)]
#[macro_use]
extern crate serde;

// macros
mod byte;

// modules
mod array;
pub mod eth;
