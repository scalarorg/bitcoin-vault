#!/bin/sh
run() {
    NAME=${1:-"bitcoin-regtest"}
    docker run --rm -d \
        --name ${NAME} \
        -p 18332:18332 \
        -p 18333:18333 \
        -v $(pwd)/.bitcoin:/root/.bitcoin \
        -v $(pwd)/bitcoin.conf:/root/.bitcoin/bitcoin.conf \
        -v $(pwd)/bitcoin.sh:/root/bitcoin.sh \
        -e DATADIR=/root/.bitcoin \
        -u root \
        -w /root/.bitcoin \
        --entrypoint /bin/sh \
        lncm/bitcoind:v25.0 /root/bitcoin.sh entrypoint
}

createwallet_descriptors() {
    WALLET_NAME=${1:-staker}
    WALLET_PASSPHRASE=${2:-passphrase}
    bitcoin-cli -named createwallet \
        wallet_name=${WALLET_NAME} \
        passphrase=${WALLET_PASSPHRASE} \
        load_on_startup=true \
        descriptors=true # Use descriptors for Taproot and P2WPKH addresses
}

getnewtaprootaddress() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    LABEL="p2tr"
    BTC_ADDRESS=$(bitcoin-cli -rpcwallet=${WALLET_NAME} getnewaddress $LABEL bech32m)
    echo $BTC_ADDRESS >$WORKDIR/${LABEL}.txt
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletpassphrase ${WALLET_PASSPHRASE:-passphrase} 60
    bitcoin-cli -rpcwallet=${WALLET_NAME} getaddressinfo $BTC_ADDRESS >$WORKDIR/${LABEL}-info.txt
    bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 101 ${BTC_ADDRESS}
}

getnewp2wpkhaddress() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    LABEL="p2wpkh"
    BTC_ADDRESS=$(bitcoin-cli -rpcwallet=${WALLET_NAME} getnewaddress $LABEL bech32)
    echo $BTC_ADDRESS >$WORKDIR/${LABEL}.txt
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletpassphrase ${WALLET_PASSPHRASE:-passphrase} 60
    bitcoin-cli -rpcwallet=${WALLET_NAME} getaddressinfo $BTC_ADDRESS >$WORKDIR/${LABEL}-info.txt
    bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 101 ${BTC_ADDRESS}
}

generate_blocks() {
    WORKDIR=${DATADIR:-/data/.bitcoin}
    while :; do
        echo "Generating blocks to p2tr"
        bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 1 $(cat $WORKDIR/p2tr.txt)
        echo "Generating blocks to p2wpkh"
        bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 1 $(cat $WORKDIR/p2wpkh.txt)
        sleep 30
    done
}

entrypoint() {
    apk add --no-cache jq
    WORKDIR=${DATADIR:-/data/.bitcoin}
    rm -rf ${WORKDIR}/p2tr*
    rm -rf ${WORKDIR}/p2wpkh*
    bitcoind
    while ! nc -z 127.0.0.1 18332; do
        sleep 1
    done
    sleep 15

    # Create wallet and addresses
    # createwallet_descriptors staker
    # getnewtaprootaddress
    # getnewp2wpkhaddress

    # generate_blocks
    sleep infinity
}

create_psbt() {
    FROM_ADDRESS=$(jq -r '.address' "$WORKDIR/root/.bitcoin/p2tr-info.txt")
    TO_ADDRESS=$(jq -r '.address' "$WORKDIR/root/.bitcoin/p2wpkh-info.txt")

    # Get the UTXO details
    UTXOS=$(bitcoin-cli -rpcwallet=staker listunspent 6 9999999 "[\"${FROM_ADDRESS}\"]")
    TXID=$(echo $UTXOS | jq -r '.[0].txid')
    VOUT=$(echo $UTXOS | jq -r '.[0].vout')

    echo "TXID: $TXID"
    echo "VOUT: $VOUT"
    echo "UTXOS[0]: $(echo $UTXOS | jq -r '.[0]')"
    echo "FROM_ADDRESS: $FROM_ADDRESS"
    echo "TO_ADDRESS: $TO_ADDRESS"

    # Create PSBT
    PSBT_RESULT=$(bitcoin-cli -rpcwallet=staker walletcreatefundedpsbt \
        "[{\"txid\":\"${TXID}\",\"vout\":${VOUT}}]" \
        "[{\"${TO_ADDRESS}\": 30}]" \
        0 '{"replaceable": true, "feeRate": 0.0001}')

    echo "PSBT Result: $PSBT_RESULT"

    bitcoin-cli -rpcwallet=staker walletpassphrase ${WALLET_PASSPHRASE:-passphrase} 60

    # Process PSBT using wallet
    SIGNED_RESULT=$(bitcoin-cli -rpcwallet=staker walletprocesspsbt $(echo $PSBT_RESULT | jq -r '.psbt'))

    echo "Signed Result: $SIGNED_RESULT"

    if [ "$(echo $SIGNED_RESULT | jq -r '.complete')" = "true" ]; then
        # If signing is complete, finalize and broadcast
        FINAL_RESULT=$(bitcoin-cli finalizepsbt $(echo $SIGNED_RESULT | jq -r '.psbt'))
        if [ "$(echo $FINAL_RESULT | jq -r '.complete')" = "true" ]; then
            # Broadcast the transaction
            TXID=$(bitcoin-cli sendrawtransaction $(echo $FINAL_RESULT | jq -r '.hex'))
            echo "Transaction broadcast: $TXID"
        else
            echo "Failed to finalize PSBT"
        fi
    else
        echo "Failed to sign PSBT completely"
    fi
}

import_private_key() {
    WALLET_NAME=${1:-staker}
    PRIVATE_KEY=${2:-"cQT95AF79E2WrWbxKEsZq3uH2GZj2gs34b7NGW8CPnL16po68CBg"}
    LABEL=${3:-"imported_key"}

    WALLET_PASSPHRASE=${WALLET_PASSPHRASE:-passphrase}
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletpassphrase ${WALLET_PASSPHRASE} 60

    # Create the ranged descriptor with the private key
    TR_DESC=$(bitcoin-cli getdescriptorinfo "tr($PRIVATE_KEY)")
    echo "TR_DESC: $TR_DESC"

    # Import the descriptor with a range
    bitcoin-cli -rpcwallet=${WALLET_NAME} importdescriptors '[
        {
            "desc": "'$(echo $TR_DESC | jq -r '.descriptor')'",
            "timestamp": "now",
            "label": "'$LABEL'_tr",
            "internal": false,
            "active": true,
            "range": [0,0]
        }
    ]'
}

p2tr() {
    WALLET_NAME=staker
    PRIVATE_KEY="cQ7kMt56n8GeKkshaiCt3Lh2ChuaD3tWdSrH37MwU93PA4qZs9JR"
    WALLET_PASSPHRASE=passphrase
    # XONLY_PUBKEY="155f0dd5185e65acd2167b9528a92b9fccaa6cf914a3c10416abd3e5de77377d"

    # Unlock the wallet
    bitcoin-cli -rpcwallet=${WALLET_NAME} walletpassphrase ${WALLET_PASSPHRASE} 60

    ORIGINAL_DESC="tr(${PRIVATE_KEY})"

    DESC_INFO=$(bitcoin-cli getdescriptorinfo "$ORIGINAL_DESC")

    echo "DESC_INFO: $DESC_INFO"

    CHECKSUM=$(echo "$DESC_INFO" | jq -r '.checksum')

    echo "CHECKSUM: $CHECKSUM"

    RESULT=$(bitcoin-cli importdescriptors '[{ "desc": "'"$ORIGINAL_DESC"'#'"$CHECKSUM"'", "timestamp": "now", "internal": true }]')

    echo "RESULT: $RESULT"

    ADDRESS_ARRAY=$(bitcoin-cli deriveaddresses "$ORIGINAL_DESC#$CHECKSUM")

    echo "Derived address: $ADDRESS_ARRAY"

    ADDRESS=$(echo $ADDRESS_ARRAY | jq -r '.[0]')

    UTXOS=$(bitcoin-cli -rpcwallet=staker listunspent 6 9999999 "[\"${ADDRESS}\"]")

    echo "Before: $UTXOS"

    ## fund the address
    bitcoin-cli -rpcwallet=${WALLET_NAME} generatetoaddress 1 $ADDRESS

    sleep 10

    UTXOS=$(bitcoin-cli -rpcwallet=staker listunspent 6 9999999 "[\"${ADDRESS}\"]")

    echo "After: $UTXOS"
}

list() {

    FROM_ADDRESS="tb1p5hpkty3ykt92qx6m0rastprnreqx6dqexagg8mgp3hgz53p9lk3qd2c4f2"

    UTXOS=$(bitcoin-cli -rpcwallet=staker listunspent 6 9999999 "[\"${FROM_ADDRESS}\"]")

    echo "UTXOS: $UTXOS"

}

$@
