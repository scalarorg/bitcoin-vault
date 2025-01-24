#!/bin/sh
run() {
    PRIVKEY=${1:-}
    NAME=${2:-"bitcoin-regtest"}
    docker rm -f ${NAME} || true
    validate_wif "$PRIVKEY"
    while [ $? -eq 0 ]; do
        echo "Invalid private key"
        read -p "Enter the private key: " answer
        PRIVKEY=$answer
        validate_wif "$PRIVKEY"
    done

    docker run --rm -d \
        --name ${NAME} \
        -p 18332:18332 \
        -p 18333:18333 \
        -v $(pwd)/.bitcoin:/root/.bitcoin \
        -v $(pwd)/bitcoin.conf:/root/.bitcoin/bitcoin.conf \
        -v $(pwd)/bitcoin.sh:/root/bitcoin.sh \
        -e DATADIR=/root/.bitcoin \
        -e BOND_HOLDER_PRIVATE_KEY=$PRIVKEY \
        -u root \
        -w /root/.bitcoin \
        --entrypoint /bin/sh \
        lncm/bitcoind:v26.1 /root/bitcoin.sh entrypoint
}

validate_wif() {
    PRIVKEY=${1:-}
    if [ -z "$PRIVKEY" ]; then
        echo "Private key is not set"
        return 0
    fi
    if ! echo "$PRIVKEY" | grep -qE '^[5KLc9][1-9A-HJ-NP-Za-km-z]{50,51}$'; then
        echo "Private key must be in WIF format"
        return 0
    fi
    return 1
}

entrypoint() {
    if [ -z "$BOND_HOLDER_PRIVATE_KEY" ]; then
        echo "BOND_HOLDER_PRIVATE_KEY is not set"
        exit 1
    fi

    apk add --no-cache jq
    WORKDIR=${DATADIR:-/data/.bitcoin}
    bitcoind
    while ! nc -z 127.0.0.1 18332; do
        sleep 1
    done

    createwallet_descriptors staker passphrase
    createwallet_legacy legacy passphrase

    # Read wif from .env
    STAKER_WIF=$BOND_HOLDER_PRIVATE_KEY

    import_wallet_by_wif staker $STAKER_WIF p2tr passphrase
    import_wallet_by_wif legacy $STAKER_WIF p2wpkh passphrase

    fund_address staker $(cat $WORKDIR/staker-p2tr.txt)
    fund_address legacy $(cat $WORKDIR/legacy-p2wpkh.txt)

    list_unspent staker $(cat $WORKDIR/staker-p2tr.txt)
    list_unspent legacy $(cat $WORKDIR/legacy-p2wpkh.txt)

    ln -s /root/bitcoin.sh /usr/local/bin/bsh

    create_miner_wallet miner passphrase

    while true; do
        MINER_ADDRESS=$(cat $WORKDIR/miner-p2tr.txt)
        echo "Mining 1 block to ${MINER_ADDRESS}"
        fund_address miner ${MINER_ADDRESS}
    done

    sleep infinity
}

createwallet_descriptors() {
    WALLET_NAME=${1:-staker}
    WALLET_PASSPHRASE=${2:-passphrase}

    bitcoin-cli -named createwallet \
        "wallet_name=${WALLET_NAME}" \
        "passphrase=${WALLET_PASSPHRASE}" \
        "load_on_startup=true" \
        "descriptors=true" # Use descriptors for Taproot and P2WPKH addresses

    echo "LISTING WALLETS"
    bitcoin-cli listwallets
}

createwallet_legacy() {
    WALLET_NAME=${1:-legacy}
    WALLET_PASSPHRASE=${2:-passphrase}
    bitcoin-cli -named createwallet \
        "wallet_name=${WALLET_NAME}" \
        "passphrase=${WALLET_PASSPHRASE}" \
        "load_on_startup=true" \
        "descriptors=false"

    echo "WALLET CREATED: ${WALLET_NAME}"
    bitcoin-cli listwallets
}

create_miner_wallet() {
    MINER_WALLET_NAME=${1:-miner}
    WALLET_PASSPHRASE=${2:-passphrase}

    bitcoin-cli -named createwallet \
        "wallet_name=${MINER_WALLET_NAME}" \
        "passphrase=${WALLET_PASSPHRASE}" \
        "load_on_startup=true" \
        "descriptors=true" # Use descriptors for Taproot and P2WPKH addresses

    # create a p2tr address

    BTC_ADDRESS=$(bitcoin-cli -rpcwallet=${MINER_WALLET_NAME} getnewaddress $LABEL bech32m)

    echo $BTC_ADDRESS >$WORKDIR/${MINER_WALLET_NAME}-p2tr.txt
}

fund_address() {
    WALLET_NAME=${1:-staker}
    ADDRESS=${2}
    bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 101 ${ADDRESS} >/dev/null 2>&1
    sleep 5
}

unlock_wallet() {
    WALLET_NAME=${1:-staker}
    WALLET_PASSPHRASE=${2:-passphrase}
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletpassphrase ${WALLET_PASSPHRASE} 60
}

import_wallet_by_wif() {
    WALLET_NAME=${1:-staker}
    WIF=${2}
    ADDRESS_TYPE=${3:-p2tr}
    WALLET_PASSPHRASE=${4:-passphrase}

    if [ -z "$WIF" ]; then
        echo "WIF is required"
        exit 1
    fi

    unlock_wallet ${WALLET_NAME} ${WALLET_PASSPHRASE}

    ADDRESS=""

    if [ "$ADDRESS_TYPE" = "p2tr" ]; then
        ADDRESS=$(p2tr ${WALLET_NAME} ${WIF})

    elif [ "$ADDRESS_TYPE" = "p2wpkh" ]; then
        ADDRESS=$(p2wpkh ${WALLET_NAME} ${WIF})
    else
        echo "Invalid address type"
        exit 1
    fi

    echo "$ADDRESS_TYPE address: $ADDRESS"
    echo $ADDRESS >$WORKDIR/${WALLET_NAME}-${ADDRESS_TYPE}.txt
}

p2tr() {
    WALLET_NAME=${1:-staker}

    WIF=${2}

    ORIGINAL_DESC="tr(${WIF})"
    DESC_INFO=$(bitcoin-cli -rpcwallet=${WALLET_NAME} getdescriptorinfo "$ORIGINAL_DESC")
    CHECKSUM=$(echo "$DESC_INFO" | jq -r '.checksum')
    RESULT=$(bitcoin-cli -rpcwallet=${WALLET_NAME} importdescriptors '[{ "desc": "'"$ORIGINAL_DESC"'#'"$CHECKSUM"'", "timestamp": "now", "internal": true }]')

    ADDRESS_ARRAY=$(bitcoin-cli -rpcwallet=${WALLET_NAME} deriveaddresses "$ORIGINAL_DESC#$CHECKSUM")

    ADDRESS=$(echo $ADDRESS_ARRAY | jq -r '.[0]')

    echo $ADDRESS
}

p2wpkh() {
    WALLET_NAME=${1:-legacy}
    WIF=${2}

    # Import the private key
    bitcoin-cli -rpcwallet=${WALLET_NAME} importprivkey "${WIF}" "label" false

    # Get all addresses with the label
    ADDRESSES=$(bitcoin-cli -rpcwallet=${WALLET_NAME} getaddressesbylabel "label")

    # Loop through addresses and find the bech32 one
    for addr in $(echo $ADDRESSES | jq -r 'keys[]'); do
        ADDR_INFO=$(bitcoin-cli -rpcwallet=${WALLET_NAME} getaddressinfo "$addr")
        IS_WITNESS=$(echo $ADDR_INFO | jq -r '.iswitness')
        WITNESS_VERSION=$(echo $ADDR_INFO | jq -r '.witness_version')

        # Check if it's a native segwit address (bech32)
        if [ "$IS_WITNESS" = "true" ] && [ "$WITNESS_VERSION" = "0" ]; then
            echo $addr
            break
        fi
    done
}

## CLI Helpers

### Usage: bsh <command>

list_descriptors() {
    bitcoin-cli listdescriptors
}

list_unspent() {
    WALLET_NAME=${1:-staker}
    ADDRESS=${2}
    bitcoin-cli -rpcwallet=${WALLET_NAME} listunspent 6 9999999 "[\"${ADDRESS}\"]"
}

getrawtx() {
    TXID=${1}
    bitcoin-cli getrawtransaction ${TXID} true
}

gettx() {
    TXID=${1}
    WALLET_NAME=${2:-staker}
    bitcoin-cli -rpcwallet=${WALLET_NAME} gettransaction ${TXID}
}

decodepsbt() {
    PSBT=${1}
    bitcoin-cli decodepsbt ${PSBT}
}

processpsbt() {
    PSBT=${1}
    WALLET_NAME=${2:-staker}
    WALLET_PASSPHRASE=${3:-passphrase}
    unlock_wallet ${WALLET_NAME} ${WALLET_PASSPHRASE}
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletprocesspsbt ${PSBT}
}

processpsbt_and_broadcast() {
    PSBT=${1}
    WALLET_NAME=${2:-staker}
    WALLET_PASSPHRASE=${3:-passphrase}
    result=$(processpsbt ${PSBT} ${WALLET_NAME} ${WALLET_PASSPHRASE})
    echo "Process Result: $result"

    if [ "$(echo $result | jq -r '.complete')" = "true" ]; then
        psbt=$(echo $result | jq -r '.psbt')
        finalize_and_broadcast ${psbt} ${WALLET_NAME} ${WALLET_PASSPHRASE}
        bitcoin-cli getrawtransaction ${txid} true
        #
    else
        echo "Failed to sign PSBT completely"
    fi
}

pab() {
    processpsbt_and_broadcast $@
}

finalize_and_broadcast() {
    PSBT=${1}
    WALLET_NAME=${2:-staker}
    WALLET_PASSPHRASE=${3:-passphrase}
    result=$(bitcoin-cli -rpcwallet=${WALLET_NAME} finalizepsbt ${PSBT})
    echo "Finalize Result: $result"
    if [ "$(echo $result | jq -r '.complete')" = "true" ]; then
        hex=$(echo $result | jq -r '.hex')
        echo "Transaction Hex: $hex"
        txid=$(bitcoin-cli sendrawtransaction ${hex})
        echo "Transaction broadcast, txid: $txid"
    else
        echo "Failed to finalize PSBT"
    fi
}

# Dont know how to use descriptors to handle dynamic scripts
import2() {
    # WALLET_NAME=${1:-protocol}
    # WIF=cVpL6mBRYV3Dmkx87wfbtZ4R3FTD6g58VkTt1ERkqGTMzTcDVw5M
    # WALLET_PASSPHRASE=${2:-passphrase}
    # NUMS=50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0
    # USER=2ae31ea8709aeda8194ba3e2f7e7e95e680e8b65135c8983c0a298d17bc5350a

    # unlock_wallet ${WALLET_NAME} ${WALLET_PASSPHRASE}

    # DESCRIPTOR="tr($NUMS,{and_v(v:pk_k($USER),pk_k(key($WIF)))})"

    # DESC_INFO=$(bitcoin-cli getdescriptorinfo "$DESCRIPTOR")
    # DESCRIPTOR_WITH_CHECKSUM=$(echo "$DESC_INFO" | jq -r '.descriptor')
    echo "Not implemented"
}
$@
