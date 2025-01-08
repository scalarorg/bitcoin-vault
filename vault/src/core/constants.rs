/**
 * ```
 * Please refer to [OP_RETURN](../docs/op_return.md) for more details.
 * ```
 */

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

/// Size of the covenant quorum field in bytes
pub const COVENANT_QUORUM_SIZE: usize = 1;

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
    + COVENANT_QUORUM_SIZE
    + DEST_CHAIN_SIZE
    + DEST_TOKEN_ADDRESS_SIZE
    + DEST_RECIPIENT_ADDRESS_SIZE;

/// Total size of the embbeded data script for unstaking
pub const UNSTAKING_EMBEDDED_DATA_SCRIPT_SIZE: usize =
    TAG_HASH_SIZE + VERSION_SIZE + NETWORK_ID_SIZE + FLAGS_SIZE + SERVICE_TAG_HASH_SIZE;
