#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![forbid(unsafe_code)]

mod cursor;
mod error;

pub use cursor::Cursor;
pub use error::BufferTooSmall;

pub use byteorder::{BigEndian, LittleEndian};
