use bitcoin::{key::Secp256k1, secp256k1::All};

pub struct VaultManager {
    secp: Secp256k1<All>,
    tag: Vec<u8>,
    version: u8,
}

impl VaultManager {
    pub fn new(tag: Vec<u8>, version: u8) -> Self {
        let secp = Secp256k1::new();
        Self { secp, tag, version }
    }

    pub fn secp(&self) -> &Secp256k1<All> {
        &self.secp
    }

    pub fn tag(&self) -> &Vec<u8> {
        &self.tag
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}
