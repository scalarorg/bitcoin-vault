mod decoder;
mod errors;
pub mod vault;
// #[cfg(test)]
// mod tests;

pub use bitcoin_vault::errors::VaultError;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
