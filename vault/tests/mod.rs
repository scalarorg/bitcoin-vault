#[cfg(test)]
pub use utils::*;

#[cfg(test)]
pub use e2e::*;

// cargo test --package bitcoin-vault --test mod -- build_spend --show-output
#[cfg(test)]
mod build_spend;

#[macro_use]
pub mod utils;

#[cfg(test)]
mod setup_suite;

#[cfg(test)]
pub use setup_suite::*;

#[cfg(test)]
mod sign_psbt;

#[cfg(test)]
mod e2e;

#[cfg(test)]
mod utxos;
