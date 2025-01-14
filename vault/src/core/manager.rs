use bitcoin::{key::Secp256k1, secp256k1::All, PublicKey, XOnlyPublicKey};

#[derive(Debug)]
pub struct VaultManager {
    secp: Secp256k1<All>,
    tag: Vec<u8>,
    service_tag: Vec<u8>,
    version: u8,
    network_id: u8,
}

#[derive(Debug)]
pub struct XOnlyKeys {
    pub user: XOnlyPublicKey,
    pub protocol: XOnlyPublicKey,
    pub covenants: Vec<XOnlyPublicKey>,
}

impl VaultManager {
    pub fn new(tag: Vec<u8>, service_tag: Vec<u8>, version: u8, network_id: u8) -> Self {
        let secp = Secp256k1::new();
        Self {
            secp,
            tag,
            service_tag,
            version,
            network_id,
        }
    }

    pub fn secp(&self) -> &Secp256k1<All> {
        &self.secp
    }

    pub fn tag(&self) -> &Vec<u8> {
        &self.tag
    }

    pub fn service_tag(&self) -> &Vec<u8> {
        &self.service_tag
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn network_id(&self) -> u8 {
        self.network_id
    }

    pub fn convert_all_to_x_only_keys(
        user_pub_key: &PublicKey,
        protocol_pub_key: &PublicKey,
        covenant_pub_keys: &[PublicKey],
    ) -> XOnlyKeys {
        let user_x_only = XOnlyPublicKey::from(*user_pub_key);
        let protocol_x_only = XOnlyPublicKey::from(*protocol_pub_key);
        let covenant_x_only = covenant_pub_keys
            .iter()
            .map(|pk| XOnlyPublicKey::from(*pk))
            .collect();

        XOnlyKeys {
            user: user_x_only,
            protocol: protocol_x_only,
            covenants: covenant_x_only,
        }
    }
}
