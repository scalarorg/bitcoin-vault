use alloy::json_abi::JsonAbi;

pub fn load_abi(path: &str) -> JsonAbi {
    let json_str = std::fs::read_to_string(
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join(path),
    )
    .expect("Failed to read ABI file");

    let abi = serde_json::from_str(&json_str).unwrap();

    abi
}

lazy_static::lazy_static! {
    pub static ref IERC20_ABI: JsonAbi = load_abi("abi/IERC20.json");
    pub static ref IGATEWAY_ABI: JsonAbi = load_abi("abi/IGateway.json");
}
