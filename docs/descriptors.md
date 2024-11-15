# Descriptors Channel logs

## 2024-11-15

- `bitcoin.sh import2`

- Currently I am trying to understand how to use descriptors to import multiple keys into a wallet.

- The idea is to have a descriptor that can sign the user_protocol branch.

- Flows:

  1. Users form the unstaking psbt in case user_protocol
  2. Sign the psbt by their wallets on website or any wallet clients
  3. Users transfer the psbt to the protocol
  4. Protocol should have the descriptor that can sign the psbt for user_protocol branch

- Current status:

  - I created a descriptor for p2tr and can sign the single psbt

  ```
  bitcoin.sh p2tr

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
  ```

- The reason why I have to stop researching is that I don't know how to use descriptors to handle dynamic scripts.

- The descriptor must form the script for user_protocol branch.
- For example, if I have 2 keys: `p2tr(key1), p2tr(key2)`, the script should be `and_v(v:pk_k(key1), pk_k(key2))` and it also has the NUMS so it can be `tr($NUMS,{and_v(v:pk_k($USER),pk_k(key($WIF)))})` or any other combination, where `key1` and `key2` are the user public keys and `WIF` is the user private key, `NUMS` is the [BIP-0341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)

- When protocol will receive the psbt, it should have the descriptor that can sign the psbt for user_protocol branch, so the protocol must have the user public keys and the number of the descriptor is infinite. (When you read to this line, please double check it, because I am not sure about this)

- I will continue researching and will update the logs.
