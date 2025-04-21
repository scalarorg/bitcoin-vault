use bitcoin::{PublicKey, XOnlyPublicKey};

pub fn convert_pubkey_to_x_only_key(pubkey: &PublicKey) -> XOnlyPublicKey {
    XOnlyPublicKey::from(*pubkey)
}
