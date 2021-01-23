//! See the parser module for functions to parse GBX files and buffers [parser](gbx::parser).
//! The datastructures used are found in [gbx](gbx).
pub mod gbx;

pub use gbx::parser::{parse_from_buffer, parse_from_file};
