.PHONY: build run send-token

build:
	cargo build --release --bin tvl_maker && cp ../target/release/tvl_maker .
send-token:
	./tvl_maker send-token -a 1000 -k 123456 -t 123456 -g 12345 -c eth -r 7890 -u  https://eth-mainnet.g.alchemy.com/v2/Iu6347YsLO-G4_Q_DwoSTms5sL6Zs5Wh
