#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

mod lowercase_string;
mod solver;
mod utils;

pub use lowercase_string::LowercaseString;
pub use solver::Solver;
