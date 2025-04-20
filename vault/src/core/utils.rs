use bitcoin::{PublicKey, XOnlyPublicKey};

use super::XOnlyKeys;

pub fn convert_pubkey_to_x_only_key(pubkey: &PublicKey) -> XOnlyPublicKey {
    XOnlyPublicKey::from(*pubkey)
}
