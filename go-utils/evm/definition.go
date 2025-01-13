package evm

import "github.com/scalarorg/bitcoin-vault/go-utils/chain"

var chains = []chainType{
	{
		BaseChain: &chain.BaseChain{
			ID: 1,
			DisplayedName: "Ethereum",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2,
			DisplayedName: "Expanse Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5,
			DisplayedName: "Goerli",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7,
			DisplayedName: "ThaiChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8,
			DisplayedName: "Ubiq Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9,
			DisplayedName: "ZKsync CLI Local Hyperchain L1",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 10,
			DisplayedName: "OP Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11,
			DisplayedName: "Metadium Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 14,
			DisplayedName: "Flare Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 15,
			DisplayedName: "Diode Prenet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 16,
			DisplayedName: "Songbird Testnet Coston",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 19,
			DisplayedName: "Songbird Canary-Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 20,
			DisplayedName: "Elastos Smart Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 21,
			DisplayedName: "Elastos Smart Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 24,
			DisplayedName: "KardiaChain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 25,
			DisplayedName: "Cronos Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 30,
			DisplayedName: "Rootstock Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 31,
			DisplayedName: "Rootstock Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 40,
			DisplayedName: "Telos",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 41,
			DisplayedName: "Telos",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42,
			DisplayedName: "LUKSO",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 44,
			DisplayedName: "Crab Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 46,
			DisplayedName: "Darwinia Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 47,
			DisplayedName: "Acria IntelliChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 50,
			DisplayedName: "XinFin Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 51,
			DisplayedName: "Apothem Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 52,
			DisplayedName: "CoinEx Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 56,
			DisplayedName: "BNB Smart Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 57,
			DisplayedName: "Syscoin Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 60,
			DisplayedName: "GoChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 61,
			DisplayedName: "Ethereum Classic",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 66,
			DisplayedName: "OKC",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 71,
			DisplayedName: "Conflux eSpace Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 74,
			DisplayedName: "IDChain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 82,
			DisplayedName: "Meter",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 83,
			DisplayedName: "Meter Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 88,
			DisplayedName: "Viction",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 89,
			DisplayedName: "Viction Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 94,
			DisplayedName: "SwissDLT Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 96,
			DisplayedName: "Bitkub",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 97,
			DisplayedName: "Binance Smart Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 98,
			DisplayedName: "Six Protocol",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 100,
			DisplayedName: "Gnosis",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 103,
			DisplayedName: "WorldLand Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 106,
			DisplayedName: "Velas EVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 108,
			DisplayedName: "ThunderCore Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 109,
			DisplayedName: "Shibarium",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 112,
			DisplayedName: "Coinbit Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 114,
			DisplayedName: "Flare Testnet Coston2",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 119,
			DisplayedName: "ENULS Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 122,
			DisplayedName: "Fuse",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 123,
			DisplayedName: "Fuse Sparknet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 133,
			DisplayedName: "HashKey Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 137,
			DisplayedName: "Polygon",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 146,
			DisplayedName: "Sonic",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 148,
			DisplayedName: "Shimmer",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 151,
			DisplayedName: "Redbelly Network Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 153,
			DisplayedName: "Redbelly Network Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 157,
			DisplayedName: "Puppynet Shibarium",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 168,
			DisplayedName: "AIOZ Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 169,
			DisplayedName: "Manta Pacific Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 179,
			DisplayedName: "ABEY Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 185,
			DisplayedName: "Mint Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 195,
			DisplayedName: "X1 Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 196,
			DisplayedName: "X Layer Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 199,
			DisplayedName: "BitTorrent",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 202,
			DisplayedName: "Edgeless Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 204,
			DisplayedName: "opBNB",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 223,
			DisplayedName: "B2",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 240,
			DisplayedName: "Nexilix Smart Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 242,
			DisplayedName: "Plinga",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 246,
			DisplayedName: "Energy Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 248,
			DisplayedName: "Oasys",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 250,
			DisplayedName: "Fantom",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 251,
			DisplayedName: "Glide L1 Protocol XP",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 252,
			DisplayedName: "Fraxtal",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 253,
			DisplayedName: "Glide L2 Protocol XP",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 254,
			DisplayedName: "Swan Chain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 255,
			DisplayedName: "Kroma",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 260,
			DisplayedName: "ZKsync InMemory Node",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 260,
			DisplayedName: "Guru Network Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 261,
			DisplayedName: "Guru Network Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 269,
			DisplayedName: "High Performance Blockchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 270,
			DisplayedName: "ZKsync CLI Local Hyperchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 270,
			DisplayedName: "ZKsync CLI Local Node",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 272,
			DisplayedName: "ZKsync CLI Local Custom Hyperchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 273,
			DisplayedName: "XR One",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 282,
			DisplayedName: "Cronos zkEVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 288,
			DisplayedName: "Boba Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 291,
			DisplayedName: "Orderly",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 295,
			DisplayedName: "Hedera Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 296,
			DisplayedName: "Hedera Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 297,
			DisplayedName: "Hedera Previewnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 300,
			DisplayedName: "ZKsync Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 311,
			DisplayedName: "Omax Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 314,
			DisplayedName: "Filecoin Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 321,
			DisplayedName: "KCC Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 324,
			DisplayedName: "ZKsync Era",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 336,
			DisplayedName: "Shiden",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 338,
			DisplayedName: "Cronos Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 360,
			DisplayedName: "Shape",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 361,
			DisplayedName: "Theta Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 365,
			DisplayedName: "Theta Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 369,
			DisplayedName: "PulseChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 388,
			DisplayedName: "Cronos zkEVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 420,
			DisplayedName: "Optimism Goerli",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 424,
			DisplayedName: "PGN",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 462,
			DisplayedName: "Areon Network Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 463,
			DisplayedName: "Areon Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 480,
			DisplayedName: "World Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 545,
			DisplayedName: "Flow EVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 570,
			DisplayedName: "Rollux Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 571,
			DisplayedName: "MetaChain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 592,
			DisplayedName: "Astar",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 595,
			DisplayedName: "Mandala TC9",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 599,
			DisplayedName: "Metis Goerli",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 646,
			DisplayedName: "Flow EVM Previewnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 686,
			DisplayedName: "Karura",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 690,
			DisplayedName: "Redstone",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 698,
			DisplayedName: "Matchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 699,
			DisplayedName: "Matchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 701,
			DisplayedName: "Koi Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 721,
			DisplayedName: "Lycan",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 747,
			DisplayedName: "Flow EVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 753,
			DisplayedName: "Rivalz",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 766,
			DisplayedName: "QL1",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 787,
			DisplayedName: "Acala",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 824,
			DisplayedName: "Daily Network Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 825,
			DisplayedName: "Daily Network Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 841,
			DisplayedName: "Taraxa Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 842,
			DisplayedName: "Taraxa Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 888,
			DisplayedName: "Wanchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 919,
			DisplayedName: "Mode Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 943,
			DisplayedName: "PulseChain V4",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 957,
			DisplayedName: "Lyra Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 995,
			DisplayedName: "5ireChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 997,
			DisplayedName: "5ireChain Thunder Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 999,
			DisplayedName: "Wanchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 999,
			DisplayedName: "Zora Goerli Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1001,
			DisplayedName: "Klaytn Baobab Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1001,
			DisplayedName: "Kairos Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1004,
			DisplayedName: "Ekta Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1017,
			DisplayedName: "BNB Greenfield Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1028,
			DisplayedName: "BitTorrent Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1030,
			DisplayedName: "Conflux eSpace",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1038,
			DisplayedName: "Bronos Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1039,
			DisplayedName: "Bronos",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1073,
			DisplayedName: "Shimmer Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1075,
			DisplayedName: "IOTA EVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1088,
			DisplayedName: "Metis",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1100,
			DisplayedName: "Dymension",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1101,
			DisplayedName: "Polygon zkEVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1111,
			DisplayedName: "WEMIX",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1112,
			DisplayedName: "WEMIX Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1116,
			DisplayedName: "Core Dao",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1123,
			DisplayedName: "B2 Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1130,
			DisplayedName: "DeFiChain EVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1131,
			DisplayedName: "DeFiChain EVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1135,
			DisplayedName: "Lisk",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1215,
			DisplayedName: "ADF Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1230,
			DisplayedName: "Ultron Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1231,
			DisplayedName: "Ultron Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1234,
			DisplayedName: "Step Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1281,
			DisplayedName: "Moonbeam Development Node",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1284,
			DisplayedName: "Moonbeam",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1285,
			DisplayedName: "Moonriver",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1287,
			DisplayedName: "Moonbase Alpha",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1301,
			DisplayedName: "Unichain Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1328,
			DisplayedName: "Sei Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1329,
			DisplayedName: "Sei Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1337,
			DisplayedName: "Localhost",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1337,
			DisplayedName: "Zhejiang",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1338,
			DisplayedName: "Elysium Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1442,
			DisplayedName: "Polygon zkEVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1453,
			DisplayedName: "MetaChain Istanbul",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1513,
			DisplayedName: "Story Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1516,
			DisplayedName: "Story Odyssey",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1559,
			DisplayedName: "Tenet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1612,
			DisplayedName: "PlayFi Albireo Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1625,
			DisplayedName: "Gravity Alpha Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1663,
			DisplayedName: "Horizen Gobi Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1666,
			DisplayedName: "Harmony One",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1686,
			DisplayedName: "Mint Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1729,
			DisplayedName: "Reya Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1750,
			DisplayedName: "Metal L2",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1890,
			DisplayedName: "LightLink Phoenix Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1891,
			DisplayedName: "LightLink Pegasus Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1946,
			DisplayedName: "Soneium Minato Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1993,
			DisplayedName: "B3 Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1994,
			DisplayedName: "Ekta",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1996,
			DisplayedName: "Sanko",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2000,
			DisplayedName: "Dogechain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2017,
			DisplayedName: "Telcoin Adiri Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2020,
			DisplayedName: "Ronin",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2021,
			DisplayedName: "Edgeware EdgeEVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2021,
			DisplayedName: "Saigon Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2022,
			DisplayedName: "Beresheet BereEVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2024,
			DisplayedName: "Swan Saturn Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2026,
			DisplayedName: "Edgeless Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2040,
			DisplayedName: "Vanar Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2192,
			DisplayedName: "SnaxChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2221,
			DisplayedName: "Kava EVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2222,
			DisplayedName: "Kava EVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2331,
			DisplayedName: "RSS3 VSL Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2340,
			DisplayedName: "Atleta Olympia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2355,
			DisplayedName: "Silicon zkEVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2358,
			DisplayedName: "Kroma Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2442,
			DisplayedName: "Polygon zkEVM Cardona",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2522,
			DisplayedName: "Fraxtal Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2525,
			DisplayedName: "inEVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2710,
			DisplayedName: "Morph Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2730,
			DisplayedName: "XR Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2810,
			DisplayedName: "Morph Holesky",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2818,
			DisplayedName: "Morph",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2882,
			DisplayedName: "Chips Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2911,
			DisplayedName: "HYCHAIN",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3068,
			DisplayedName: "Bifrost Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3109,
			DisplayedName: "SatoshiVM Alpha Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3110,
			DisplayedName: "SatoshiVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3141,
			DisplayedName: "Filecoin Hyperspace",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3441,
			DisplayedName: "Manta Pacific Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3441,
			DisplayedName: "Manta Pacific Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3636,
			DisplayedName: "Botanix Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3737,
			DisplayedName: "Crossbell",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3776,
			DisplayedName: "Astar zkEVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3939,
			DisplayedName: "DOS Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3993,
			DisplayedName: "APEX Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4002,
			DisplayedName: "Fantom Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4090,
			DisplayedName: "Oasis Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4200,
			DisplayedName: "Merlin",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4201,
			DisplayedName: "LUKSO Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4202,
			DisplayedName: "Lisk Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4242,
			DisplayedName: "Nexi",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4337,
			DisplayedName: "Beam",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4460,
			DisplayedName: "Orderly Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4689,
			DisplayedName: "IoTeX",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4690,
			DisplayedName: "IoTeX Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4759,
			DisplayedName: "MEVerse Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4777,
			DisplayedName: "BlackFort Exchange Network Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4801,
			DisplayedName: "World Chain Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 4999,
			DisplayedName: "BlackFort Exchange Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5000,
			DisplayedName: "Mantle",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5001,
			DisplayedName: "Mantle Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5003,
			DisplayedName: "Mantle Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5112,
			DisplayedName: "Ham",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5115,
			DisplayedName: "Citrea Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5165,
			DisplayedName: "Bahamut",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5234,
			DisplayedName: "Humanode",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5330,
			DisplayedName: "Superseed",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5464,
			DisplayedName: "Saga",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5551,
			DisplayedName: "Nahmii 2 Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5611,
			DisplayedName: "opBNB Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5700,
			DisplayedName: "Syscoin Tanenbaum Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5858,
			DisplayedName: "Chang Chain Foundation Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 6000,
			DisplayedName: "BounceBit Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 6001,
			DisplayedName: "BounceBit Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 6038,
			DisplayedName: "Astar zkEVM Testnet zKyoto",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 6969,
			DisplayedName: "Tomb Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7000,
			DisplayedName: "ZetaChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7001,
			DisplayedName: "ZetaChain Athens Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7070,
			DisplayedName: "Planq Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7171,
			DisplayedName: "Bitrock Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7200,
			DisplayedName: "exSat Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7234,
			DisplayedName: "InitVerse Genesis Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7332,
			DisplayedName: "Horizen EON",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7518,
			DisplayedName: "MEVerse Chain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7560,
			DisplayedName: "Cyber",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7668,
			DisplayedName: "The Root Network",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7672,
			DisplayedName: "The Root Network - Porcini",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7700,
			DisplayedName: "Canto",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7887,
			DisplayedName: "Kinto Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7979,
			DisplayedName: "DOS Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8017,
			DisplayedName: "iSunCoin Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8082,
			DisplayedName: "Shardeum Sphinx",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8217,
			DisplayedName: "Kaia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8217,
			DisplayedName: "Klaytn",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8333,
			DisplayedName: "B3",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8408,
			DisplayedName: "Zenchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8428,
			DisplayedName: "THAT Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8453,
			DisplayedName: "Base",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8668,
			DisplayedName: "Hela Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8822,
			DisplayedName: "IOTA EVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8866,
			DisplayedName: "SuperLumio",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8880,
			DisplayedName: "Unique Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8881,
			DisplayedName: "Quartz Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8882,
			DisplayedName: "Opal Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 8899,
			DisplayedName: "JIBCHAIN L1",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9000,
			DisplayedName: "Evmos Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9001,
			DisplayedName: "Evmos",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9496,
			DisplayedName: "WeaveVM Alphanet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9700,
			DisplayedName: "OORT MainnetDev",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 9999,
			DisplayedName: "Fluence",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 10200,
			DisplayedName: "Gnosis Chiado",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 10242,
			DisplayedName: "Arthera",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11011,
			DisplayedName: "Shape Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11100,
			DisplayedName: "Bool Beta Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11124,
			DisplayedName: "Abstract Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11155,
			DisplayedName: "Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11235,
			DisplayedName: "HAQQ Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11297,
			DisplayedName: "Palm Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11297,
			DisplayedName: "Palm",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11501,
			DisplayedName: "BEVM Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11822,
			DisplayedName: "Artela Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12306,
			DisplayedName: "Fibo Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12323,
			DisplayedName: "Huddle01 dRTC Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12324,
			DisplayedName: "L3X Protocol",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12325,
			DisplayedName: "L3X Protocol Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12553,
			DisplayedName: "RSS3 VSL Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13001,
			DisplayedName: "SnaxChain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13337,
			DisplayedName: "Beam Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13370,
			DisplayedName: "Cannon",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13371,
			DisplayedName: "Immutable zkEVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13381,
			DisplayedName: "Phoenix Blockchain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 13473,
			DisplayedName: "Immutable zkEVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 14853,
			DisplayedName: "Humanode Testnet 5",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 15551,
			DisplayedName: "LoopNetwork Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 15557,
			DisplayedName: "EOS EVM Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 16507,
			DisplayedName: "Genesys Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 17000,
			DisplayedName: "Holesky",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 17069,
			DisplayedName: "Garnet Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 17777,
			DisplayedName: "EOS EVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 18233,
			DisplayedName: "Unreal",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 21000,
			DisplayedName: "Corn Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 21000,
			DisplayedName: "Corn",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 22222,
			DisplayedName: "Nautilus Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 22776,
			DisplayedName: "MAP Protocol",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 23294,
			DisplayedName: "Oasis Sapphire",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 23295,
			DisplayedName: "Oasis Sapphire Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 23451,
			DisplayedName: "DreyerX Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 23452,
			DisplayedName: "DreyerX Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 25925,
			DisplayedName: "Bitkub Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 28882,
			DisplayedName: "Boba Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 29112,
			DisplayedName: "HYCHAIN Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 29548,
			DisplayedName: "MCH Verse",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 31337,
			DisplayedName: "Foundry",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 31337,
			DisplayedName: "Anvil",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 31337,
			DisplayedName: "Hardhat",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 32520,
			DisplayedName: "Bitgert Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 32659,
			DisplayedName: "Fusion Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 32769,
			DisplayedName: "Zilliqa",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 33101,
			DisplayedName: "Zilliqa Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 33111,
			DisplayedName: "Curtis",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 33139,
			DisplayedName: "Ape Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 33979,
			DisplayedName: "Funki",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 34443,
			DisplayedName: "Mode Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 35441,
			DisplayedName: "Q Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 35443,
			DisplayedName: "Q Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 41455,
			DisplayedName: "Aleph Zero",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42161,
			DisplayedName: "Arbitrum One",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42170,
			DisplayedName: "Arbitrum Nova",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42220,
			DisplayedName: "Celo",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42420,
			DisplayedName: "AssetChain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42421,
			DisplayedName: "AssetChain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42766,
			DisplayedName: "ZKFair Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 42793,
			DisplayedName: "Etherlink",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 43113,
			DisplayedName: "Avalanche Fuji",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 43114,
			DisplayedName: "Avalanche",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 43851,
			DisplayedName: "ZKFair Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 44787,
			DisplayedName: "Alfajores",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 46688,
			DisplayedName: "Fusion Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 47763,
			DisplayedName: "Neo X Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 47805,
			DisplayedName: "REI Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 48899,
			DisplayedName: "Zircuit Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 48900,
			DisplayedName: "Zircuit Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 50005,
			DisplayedName: "Yooldo Verse",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 50006,
			DisplayedName: "Yooldo Verse Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 50104,
			DisplayedName: "Sophon",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 52014,
			DisplayedName: "Electroneum Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 52164,
			DisplayedName: "Fluence Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 53302,
			DisplayedName: "Superseed Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 53457,
			DisplayedName: "DODOchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 53935,
			DisplayedName: "DFK Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 54211,
			DisplayedName: "HAQQ Testedge 2",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 55244,
			DisplayedName: "Superposition",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 57000,
			DisplayedName: "Rollux Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 57073,
			DisplayedName: "Ink",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 58008,
			DisplayedName: "PGN ",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 59140,
			DisplayedName: "Linea Goerli Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 59140,
			DisplayedName: "Linea Goerli Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 59141,
			DisplayedName: "Linea Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 59144,
			DisplayedName: "Linea Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 60808,
			DisplayedName: "BOB",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 61166,
			DisplayedName: "Treasure",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 62049,
			DisplayedName: "Optopia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 62050,
			DisplayedName: "Optopia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 62092,
			DisplayedName: "TikTrix Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 63157,
			DisplayedName: "Geist Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 64165,
			DisplayedName: "Sonic Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 64240,
			DisplayedName: "Fantom Sonic Open Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 66665,
			DisplayedName: "Creator",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 71402,
			DisplayedName: "Godwoken Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 80001,
			DisplayedName: "Polygon Mumbai",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 80002,
			DisplayedName: "Polygon Amoy",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 80084,
			DisplayedName: "Berachain bArtio",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 80085,
			DisplayedName: "Berachain Artio",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 81457,
			DisplayedName: "Blast",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 84531,
			DisplayedName: "Base Goerli",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 84532,
			DisplayedName: "Base Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 88882,
			DisplayedName: "Chiliz Spicy Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 88888,
			DisplayedName: "Chiliz Chain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 88991,
			DisplayedName: "Jibchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 98864,
			DisplayedName: "Plume Devnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 98865,
			DisplayedName: "Plume Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 100009,
			DisplayedName: "Vechain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 105105,
			DisplayedName: "Stratis Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 111188,
			DisplayedName: "re.al",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 111557,
			DisplayedName: "Cyber Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 123420,
			DisplayedName: "Fluence Stage",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 124832,
			DisplayedName: "Mitosis Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 128123,
			DisplayedName: "Etherlink Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 161221,
			DisplayedName: "Plume Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 167000,
			DisplayedName: "Taiko Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 167005,
			DisplayedName: "Taiko (Alpha-3 Testnet)",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 167007,
			DisplayedName: "Taiko Jolnir (Alpha-5 Testnet)",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 167008,
			DisplayedName: "Taiko Katla (Alpha-6 Testnet)",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 167009,
			DisplayedName: "Taiko Hekla L2",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 168587,
			DisplayedName: "Blast Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 200810,
			DisplayedName: "Bitlayer Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 200810,
			DisplayedName: "Bitlayer Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 200901,
			DisplayedName: "Bitlayer",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 200901,
			DisplayedName: "Bitlayer Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 205205,
			DisplayedName: "Auroria Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 245022,
			DisplayedName: "Neon EVM MainNet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 245022,
			DisplayedName: "Neon EVM DevNet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 309075,
			DisplayedName: "One World Chain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 314159,
			DisplayedName: "Filecoin Calibration",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 360890,
			DisplayedName: "LAVITA Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 421613,
			DisplayedName: "Arbitrum Goerli",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 421614,
			DisplayedName: "Arbitrum Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 513100,
			DisplayedName: "DisChain",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 531050,
			DisplayedName: "Sophon Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 534351,
			DisplayedName: "Scroll Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 534352,
			DisplayedName: "Scroll",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 555888,
			DisplayedName: "DustBoy IoT",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 631571,
			DisplayedName: "Polter Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 641230,
			DisplayedName: "Bear Network Chain Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 660279,
			DisplayedName: "Xai Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 666666,
			DisplayedName: "Vision Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 713715,
			DisplayedName: "Sei Devnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 743111,
			DisplayedName: "Hemi Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 751230,
			DisplayedName: "Bear Network Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 752025,
			DisplayedName: "Ternoa",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 763373,
			DisplayedName: "Ink Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 808813,
			DisplayedName: "BOB Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 810180,
			DisplayedName: "zkLink Nova",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 810181,
			DisplayedName: "zkLink Nova Sepolia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 839999,
			DisplayedName: "exSat Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 888888,
			DisplayedName: "Vision",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 911867,
			DisplayedName: "Odyssey Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 978658,
			DisplayedName: "Treasure Topaz Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 984122,
			DisplayedName: "Forma",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 984123,
			DisplayedName: "Forma Sketchpad",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2524852,
			DisplayedName: "Huddle01 dRTC Chain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 3397901,
			DisplayedName: "Funki Sepolia Sandbox",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 5201420,
			DisplayedName: "Electroneum Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7225878,
			DisplayedName: "Saakuru Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7777777,
			DisplayedName: "Zora",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 10241024,
			DisplayedName: "AlienX Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 10241025,
			DisplayedName: "ALIENX Hal Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 11155420,
			DisplayedName: "OP Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 12227332,
			DisplayedName: "Neo X Testnet T4",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 20241133,
			DisplayedName: "Swan Proxima Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 22052002,
			DisplayedName: "Excelon Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 28122024,
			DisplayedName: "Ancient8 Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 41144114,
			DisplayedName: "Otim Devnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 333000333,
			DisplayedName: "Meld",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 666666666,
			DisplayedName: "Degen",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 728126428,
			DisplayedName: "Tron",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 888888888,
			DisplayedName: "Ancient8",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 994873017,
			DisplayedName: "Lumia Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 999999999,
			DisplayedName: "Zora Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1313161554,
			DisplayedName: "Aurora",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1313161555,
			DisplayedName: "Aurora Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1722641160,
			DisplayedName: "Silicon Sepolia zkEVM",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1802203764,
			DisplayedName: "Kakarot Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 1952959480,
			DisplayedName: "Lumia Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 7078815900,
			DisplayedName: "Mekong Pectra Devnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 37714555429,
			DisplayedName: "Xai Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 383414847825,
			DisplayedName: "Zeniq Mainnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 920637907288165,
			DisplayedName: "Kakarot Starknet Sepolia",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2713017997578000,
			DisplayedName: "Dchain Testnet",
		},
	},
	{
		BaseChain: &chain.BaseChain{
			ID: 2716446429837000,
			DisplayedName: "Dchain",
		},
	},
}
