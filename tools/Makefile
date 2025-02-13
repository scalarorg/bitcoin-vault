.PHONY: build run send-token

build:
	cargo build --release --bin tvl_maker && cp ../target/release/tvl_maker .
send-token:
	./tvl_maker send-token -a 1000 -k 123456 -t 123456 -g 12345 -c eth -r 7890 -u  https://eth-mainnet.g.alchemy.com/v2/Iu6347YsLO-G4_Q_DwoSTms5sL6Zs5Wh

run:
	docker run -d \
    --name=sqlitebrowser \
    -e PUID=1000 \
    -e PGID=1000 \
    -e TZ=Etc/UTC \
    -p 3000:3000 \
    -p 3001:3001 \
    -v /Users/david/Workspaces/CodeLight/scalar-bridge/bitcoin-vault/tools/tvl_maker.db:/config/tvl_maker.db:z \
    --restart unless-stopped \
    lscr.io/linuxserver/sqlitebrowser:latest