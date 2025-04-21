use bitcoin::{PublicKey, XOnlyPublicKey};

pub fn convert_pubkey_to_x_only_key(pubkey: &PublicKey) -> XOnlyPublicKey {
    XOnlyPublicKey::from(*pubkey)
}

pub fn convert_pubkeys_to_x_only_keys(pub_keys: &[PublicKey]) -> Vec<XOnlyPublicKey> {
    pub_keys.iter().map(convert_pubkey_to_x_only_key).collect()
}
