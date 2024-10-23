use bitcoin_vault::Config;

fn main() {
    println!("Hello, world!");
    let config = Config::new(None);
    println!("{:?}", config);
}
