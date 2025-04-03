pub mod error;
pub(crate) mod parser;
pub(crate) mod system;
pub(crate) mod types;

//  Re exports
pub use parser::gcode;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
