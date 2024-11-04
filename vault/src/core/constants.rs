pub const NUM_OUTPUTS: usize = 2;
pub const TAG_HASH_SIZE: usize = 4;
pub const VERSION_SIZE: usize = 1;
pub const CHAIN_ID_SIZE: usize = 8;
pub const ADDRESS_SIZE: usize = 20;

pub const EMBEDDED_DATA_SCRIPT_SIZE: usize =
    TAG_HASH_SIZE + VERSION_SIZE + CHAIN_ID_SIZE + ADDRESS_SIZE * 2;
