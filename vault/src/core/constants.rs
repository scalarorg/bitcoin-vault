/**
 * Please refer to [OP_RETURN](../docs/op_return.md) for more details.
 */
/// Size of the hash size in bytes
pub const HASH_SIZE: usize = 32;

pub const SEQUENCE_SIZE: usize = 8;
/// Size of the evm address in bytes
pub const EVM_ADDRESS_SIZE: usize = 20;

/// Size of the tag hash in bytes
pub const TAG_HASH_SIZE: usize = 6;

/// Size of the version field in bytes
pub const VERSION_SIZE: usize = 1;

/// Size of the network identifier in bytes
pub const NETWORK_ID_SIZE: usize = 1;

/// Size of the flags field in bytes
pub const FLAGS_SIZE: usize = 1;

/// Size of the optional service tag hash in bytes
pub const SERVICE_TAG_HASH_SIZE: usize = 5;

/// Size of the custodian quorum field in bytes
pub const CUSTODIAN_QUORUM_SIZE: usize = 1;

/// Size of the destination chain identifier in bytes
pub const DEST_CHAIN_SIZE: usize = 8;

/// Size of the destination token address in bytes
pub const DEST_TOKEN_ADDRESS_SIZE: usize = 20;

/// Size of the destination recipient address in bytes
pub const DEST_RECIPIENT_ADDRESS_SIZE: usize = 20;

/// Total size of the embedded data script, calculated as the sum of all component sizes
pub const EMBEDDED_DATA_SCRIPT_SIZE: usize = TAG_HASH_SIZE
    + VERSION_SIZE
    + NETWORK_ID_SIZE
    + FLAGS_SIZE
    + SERVICE_TAG_HASH_SIZE
    + CUSTODIAN_QUORUM_SIZE
    + DEST_CHAIN_SIZE
    + DEST_TOKEN_ADDRESS_SIZE
    + DEST_RECIPIENT_ADDRESS_SIZE;

/// Total size of the embbeded data script for unlocking
pub const UNLOCKING_EMBEDDED_DATA_SCRIPT_SIZE: usize = TAG_HASH_SIZE
    + VERSION_SIZE
    + NETWORK_ID_SIZE
    + FLAGS_SIZE
    + SERVICE_TAG_HASH_SIZE
    + SEQUENCE_SIZE
    + HASH_SIZE;

/*
    FEE CALCULATION
*/

pub const P2TR_INPUT_SIZE: u64 = 58; // 57.5
pub const P2TR_OUTPUT_SIZE: u64 = 43;
pub const P2TR_BUFFER_SIZE: u64 = 11; // 10.5
pub const ESTIMATE_SIGNATURE_COST: u64 = 16;
pub const ESTIMATE_ADDITIONAL_P2TR_SCRIPT_PATH_COST: u64 = 60;
