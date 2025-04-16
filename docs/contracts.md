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

Given Alice want to redeem the staked bitcoin, she have to unlock the tap tree by the following:
