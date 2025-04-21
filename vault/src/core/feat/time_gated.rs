use bitcoin::{Psbt, PublicKey};

use crate::{
    convert_pubkey_to_x_only_key, convert_pubkeys_to_x_only_keys, get_global_secp, CoreError,
    LockingScript, TaprootTree, TimeGated, TimeGatedTree, TimeGatedUnlockingParams, VaultManager,
};

impl TimeGated for VaultManager {
    type Error = CoreError;

    fn build_locking_psbt(
        &self,
        params: &crate::TimeGatedLockingParams,
    ) -> Result<Psbt, Self::Error> {
        let locking_script = <Self as TimeGated>::locking_script(
            &params.party_pubkey,
            &params.custodian_pub_keys,
            params.custodian_quorum,
            params.sequence,
        )?;

        unimplemented!()
    }

    fn locking_script(
        party: &PublicKey,
        custodian_pub_keys: &[PublicKey],
        custodian_quorum: u8,
        sequence: i64,
    ) -> Result<LockingScript, Self::Error> {
        let secp = get_global_secp();
        let keys = convert_pubkeys_to_x_only_keys(custodian_pub_keys);
        let party = convert_pubkey_to_x_only_key(party);

        let tree =
            TaprootTree::<TimeGatedTree>::new(secp, &party, &keys, custodian_quorum, sequence)?;

        Ok(LockingScript(tree.into_script(secp)))
    }

    fn build_unlocking_psbt(
        &self,
        params: &TimeGatedUnlockingParams,
    ) -> Result<bitcoin::Psbt, Self::Error> {
        unimplemented!()
    }
}
