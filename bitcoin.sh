#!/bin/sh
# wget https://bitcoincore.org/bin/bitcoin-core-26.0/bitcoin-26.0-x86_64-linux-gnu.tar.gz -o /root/bitcoin-26.0-x86_64-linux-gnu.tar.gz
# wget https://bitcoincore.org/bin/bitcoin-core-26.0/bitcoin-26.0-x86_64-linux-gnu.tar.gz
# tar -xvf bitcoin-26.0-x86_64-linux-gnu.tar.gz
# sudo ln -sf bitcoin-26.0/bin/bitcoind /usr/bin/bitcoind
# bitcoind
COVENANT_COUNT=5
start_bitcoind() {
    bitcoind -testnet \
      -rpcbind=${RPC_BIND:-127.0.0.1:18332} \
      -rpcuser=${RPC_USER:-user} \
      -rpcpassword=${RPC_PASS:-password} \
      -rpcallowip=${RPC_ALLOWIP:-127.0.0.1/0} \
      -datadir=${DATADIR:-/data/.bitcoin} \
      -server=${SERVER:-1} \
      -txindex=${TXINDEX:-1} \
      -connect=${CONNECT:-0} \
      -daemon=${DAEMON:-1}
}
createwallet_legacy() {
    WALLET_LEGACY_NAME=${1:-legacy}
    WALLET_LEACY_PASSPHRASE=${2:-passphrase}
    bitcoin-cli -named createwallet \
        wallet_name=${WALLET_LEGACY_NAME:-name} \
        passphrase=${WALLET_LEACY_PASSPHRASE:-passphrase} \
        load_on_startup=true \
        descriptors=false # create legacy wallet
}

createwallet_descriptors() {
    WALLET_DESCRIPTORS_NAME=${1:-descriptors}
    WALLET_DESCRIPTORS_PASSPHRASE=${2:-passphrase}
    bitcoin-cli -named createwallet \
        wallet_name=${WALLET_DESCRIPTORS_NAME:-name} \
        passphrase=${WALLET_DESCRIPTORS_PASSPHRASE:-passphrase} \
        load_on_startup=false \
        descriptors=true # create legacy wallet
}

getnewlegacyaddress() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    ADDRESS_TYPE=${1:-legacy}
    NAME=${2:-${ADDRESS_TYPE}}
    LABEL="address-${ADDRESS_TYPE}-${NAME}"
    BTC_ADDRESS=$(bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} getnewaddress $LABEL $ADDRESS_TYPE)
    echo $BTC_ADDRESS>$WORKDIR/${LABEL}.txt
    bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} walletpassphrase ${WALLET_LEACY_PASSPHRASE:-passphrase} 60
    bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} getaddressinfo $BTC_ADDRESS>$WORKDIR/${LABEL}-info.txt
    bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} dumpprivkey $BTC_ADDRESS>$WORKDIR/${LABEL}-privkey.txt
    bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} generatetoaddress 101 ${BTC_ADDRESS}
}

getnewtaprootaddress() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    ADDRESS_TYPE=bech32m
    NAME=${1:-${ADDRESS_TYPE}}
    LABEL="address-${ADDRESS_TYPE}-${NAME}"
    BTC_ADDRESS=$(bitcoin-cli -rpcwallet=${WALLET_DESCRIPTORS_NAME} getnewaddress $LABEL $ADDRESS_TYPE)
    echo $BTC_ADDRESS>$WORKDIR/${LABEL}.txt
    bitcoin-cli -rpcwallet=${WALLET_DESCRIPTORS_NAME} walletpassphrase ${WALLET_DESCRIPTORS_PASSPHRASE:-passphrase} 60
    bitcoin-cli -rpcwallet=${WALLET_DESCRIPTORS_NAME} getaddressinfo $BTC_ADDRESS>$WORKDIR/${LABEL}-info.txt
    # Only legacy addresses can be dumped
    # bitcoin-cli -rpcwallet=${WALLET_DESCRIPTORS_NAME} dumpprivkey $BTC_ADDRESS>$WORKDIR/${LABEL}-privkey.txt
    bitcoin-cli -rpcwallet=${WALLET_DESCRIPTORS_NAME} generatetoaddress 101 ${BTC_ADDRESS}
}

create_envs() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    ENV_FILE=${WORKDIR}/.env.btc
    STAKER_ADDRESS=$(cat ${WORKDIR}/address-bech32-staker.txt)
    STAKER_PRIVKEY=$(cat ${WORKDIR}/address-bech32-staker-privkey.txt)
    STAKER_PUBKEY=$(cat ${WORKDIR}/address-bech32-staker-info.txt | jq -r '.pubkey')
    SERVICE_ADDRESS=$(cat ${WORKDIR}/address-bech32-service.txt)
    SERVICE_PRIVKEY=$(cat ${WORKDIR}/address-bech32-service-privkey.txt)
    SERVICE_PUBKEY=$(cat ${WORKDIR}/address-bech32-service-info.txt | jq -r '.pubkey')
    PROTOCOL_ADDRESS=$(cat ${WORKDIR}/address-bech32-protocol.txt)
    PROTOCOL_PRIVKEY=$(cat ${WORKDIR}/address-bech32-protocol-privkey.txt)
    PROTOCOL_PUBKEY=$(cat ${WORKDIR}/address-bech32-protocol-info.txt | jq -r '.pubkey')
    echo "BOND_HOLDER_ADDRESS=${STAKER_ADDRESS}" > ${ENV_FILE}
    echo "BOND_HOLDER_PRIVATE_KEY=${STAKER_PRIVKEY}" >> ${ENV_FILE}
    echo "BOND_HOLDER_PUBLIC_KEY=${STAKER_PUBKEY}" >> ${ENV_FILE}
    echo "PROTOCOL_ADDRESS=${PROTOCOL_ADDRESS}" >> ${ENV_FILE}
    echo "PROTOCOL_PUBLIC_KEY=${PROTOCOL_PUBKEY}" >> ${ENV_FILE}
    echo "PROTOCOL_PRIVATE_KEY=${PROTOCOL_PRIVKEY}" >> ${ENV_FILE}
    echo "SERVICE_ADDRESS=${SERVICE_ADDRESS}" >> ${ENV_FILE}
    echo "SERVICE_PUBKEY=${SERVICE_PUBKEY}" >> ${ENV_FILE}

    COVENANT_PUBKEYS=$(cat ${WORKDIR}/address-bech32-covenant1-info.txt | jq -r '.pubkey')
    COVENANT_PRIVKEYS=$(cat ${WORKDIR}/address-bech32-covenant1-privkey.txt)
    for i in $(seq 2 ${COVENANT_COUNT:-5})
	do
        PUBKEY=$(cat ${WORKDIR}/address-bech32-covenant${i}-info.txt | jq -r '.pubkey')
        PRIVKEY=$(cat ${WORKDIR}/address-bech32-covenant${i}-privkey.txt)
        COVENANT_PUBKEYS="${COVENANT_PUBKEYS},${PUBKEY}"
        COVENANT_PRIVKEYS="${COVENANT_PRIVKEYS},${PRIVKEY}"
	done
    echo "COVENANT_PUBKEYS=${COVENANT_PUBKEYS}" >> ${ENV_FILE}
    echo "COVENANT_PRIVKEYS=${COVENANT_PRIVKEYS}" >> ${ENV_FILE}
}
 # create block for each 30s
generate_blocks() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    while :
    do
        bitcoin-cli -rpcwallet=${WALLET_LEGACY_NAME} -generate 1
        sleep 30
    done
}
entrypoint() {
    apk add --no-cache jq
    WORKDIR=${DATADIR:-/data/.bitcoin}
    rm -rf ${WORKDIR}/address-*
    bitcoind
    while ! nc -z 127.0.0.1 18332; do
        sleep 1
    done
    sleep 15
    createwallet_descriptors
    # bitcoin-cli loadwallet ${WALLET_DESCRIPTORS_NAME}
    # Taproot
    getnewtaprootaddress staker
    getnewtaprootaddress service
    getnewtaprootaddress protocol

    for i in $(seq ${COVENANT_COUNT:-5})
    do
        getnewtaprootaddress bech32 covenant${i}
    done

    createwallet_legacy
    # bitcoin-cli loadwallet ${WALLET_LEGACY_NAME}
    # getnewlegacyaddress legacy
    # getnewlegacyaddress p2sh-segwit
    # P2WSH
    getnewlegacyaddress bech32 staker
    getnewlegacyaddress bech32 service
    getnewlegacyaddress bech32 protocol

    for i in $(seq ${COVENANT_COUNT:-5})
    do
        getnewlegacyaddress bech32 covenant${i}
    done
    create_envs
    # create_envs_from_template
    generate_blocks
    sleep infinity
}

$@   