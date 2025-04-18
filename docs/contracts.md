# Contracts

## HTLC:
Let's say Alice want to swap bitcoin and eth with Bob. 
- Alice create a hash (preimage is `secret`) lock
- Alice send utxo to Bob with that lock with `locktime`. If `locktime` is not reached, Bob can spend it.
- Bob trust Alice so send eth to HTLC contract by the following hash on the ethereum chain
- Alice spend the amount of eth first on the ethereum chain and reveal the secret as well.
- Bob uses the secret to spend the utxo on the bitcoin chain.

Note:
- `hash = hash_function(secret)`
- `locktime` is the time when Bob can spend the utxo on the bitcoin chain.

In terms of bitcoin, we can operate `OP_CHECKLOCKTIMEVERIFY` and `OP_CHECKSEQUENCEVERIFY` to implement the HTLC along with Taproot mechanism:

- ***Escrow with Timeout***
An escrow that times out automatically 30 days after being funded can be established in the following way. Alice, Bob and Escrow create a 2-of-3 address with the following redeemscript.

    ```asm
        IF
            2 <Alice's pubkey> <Bob's pubkey> <Escrow's pubkey> 3 CHECKMULTISIG
        ELSE
            "30d" CHECKSEQUENCEVERIFY DROP
            <Alice's pubkey> CHECKSIG
        ENDIF
    ```
At any time funds can be spent using signatures from any two of Alice, Bob or the Escrow.
After 30 days Alice can sign alone.
The clock does not start ticking until the payment to the escrow address confirms.

- Credit: [BIP-0112](https://github.com/bitcoin/bips/blob/master/bip-0112.mediawiki)