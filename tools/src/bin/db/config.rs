use super::DbEntity;

#[derive(Debug)]
pub struct Config {
    pub id: Option<i64>,
    pub name: String,
    pub btc_node_address: String,
    pub btc_node_user: String,
    pub btc_node_password: String,
    pub btc_node_wallet: String,
    pub protocol_private_key: String,
    pub custodian_private_keys: String,
    pub custodian_quorum: u32,
    pub network: String,
    pub tag: String,
    pub version: u32,
    pub mnemonic: String,
    pub destination_chain: String,
    pub destination_token_address: String,
    pub destination_recipient_address: String,
}

impl Config {
    pub fn new(
        name: String,
        btc_node_address: String,
        btc_node_user: String,
        btc_node_password: String,
        btc_node_wallet: String,
        protocol_private_key: String,
        custodian_private_keys: Vec<String>,
        custodian_quorum: u32,
        network: String,
        tag: String,
        version: u32,
        mnemonic: String,
        destination_chain: String,
        destination_token_address: String,
        destination_recipient_address: String,
    ) -> Self {
        Self {
            id: None,
            name,
            btc_node_address,
            btc_node_user,
            btc_node_password,
            btc_node_wallet,
            protocol_private_key,
            custodian_private_keys: custodian_private_keys.join(","),
            custodian_quorum,
            network,
            tag,
            version,
            mnemonic,
            destination_chain,
            destination_token_address,
            destination_recipient_address,
        }
    }
}

impl DbEntity for Config {
    fn table_name() -> &'static str {
        "configs"
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "name",
            "btc_node_address",
            "btc_node_user",
            "btc_node_password",
            "btc_node_wallet",
            "protocol_private_key",
            "custodian_private_keys",
            "custodian_quorum",
            "network",
            "tag",
            "version",
            "mnemonic",
            "destination_chain",
            "destination_token_address",
            "destination_recipient_address",
        ]
    }

    fn values(&self) -> Vec<Box<dyn rusqlite::ToSql>> {
        vec![
            Box::new(self.name.clone()),
            Box::new(self.btc_node_address.clone()),
            Box::new(self.btc_node_user.clone()),
            Box::new(self.btc_node_password.clone()),
            Box::new(self.btc_node_wallet.clone()),
            Box::new(self.protocol_private_key.clone()),
            Box::new(self.custodian_private_keys.clone()),
            Box::new(self.custodian_quorum),
            Box::new(self.network.clone()),
            Box::new(self.tag.clone()),
            Box::new(self.version),
            Box::new(self.mnemonic.clone()),
            Box::new(self.destination_chain.clone()),
            Box::new(self.destination_token_address.clone()),
            Box::new(self.destination_recipient_address.clone()),
        ]
    }

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Config {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            btc_node_address: row.get(2)?,
            btc_node_user: row.get(3)?,
            btc_node_password: row.get(4)?,
            btc_node_wallet: row.get(5)?,
            protocol_private_key: row.get(6)?,
            custodian_private_keys: row.get(7)?,
            custodian_quorum: row.get(8)?,
            network: row.get(9)?,
            tag: row.get(10)?,
            version: row.get(11)?,
            mnemonic: row.get(12)?,
            destination_chain: row.get(13)?,
            destination_token_address: row.get(14)?,
            destination_recipient_address: row.get(15)?,
        })
    }
}
