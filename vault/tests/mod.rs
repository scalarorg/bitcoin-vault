#[cfg(test)]
pub use utils::*;

#[cfg(test)]
mod parse;

// cargo test --package bitcoin-vault --test mod -- build_spend --show-output
#[cfg(test)]
mod build_spend;

#[macro_use]
pub mod utils;
