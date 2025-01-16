use bitcoin::{
    opcodes::all::{OP_CHECKSIG, OP_CHECKSIGADD, OP_CHECKSIGVERIFY, OP_GREATERTHANOREQUAL},
    script, ScriptBuf, XOnlyPublicKey,
};

use super::{
    BuildCustodianAndPartyBranch, BuildCustodianOnlyBranch, BuildTwoPartyBranch, CoreError,
};

pub type TwoPartyBranch = ScriptBuf;
pub type CustodianAndPartyBranch = ScriptBuf;
pub type CustodianOnlyBranch = ScriptBuf;
pub type CustodianScript = ScriptBuf;
pub struct CustodianScriptBuilder;

impl CustodianScriptBuilder {
    pub fn build(
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
        initial_key: Option<&XOnlyPublicKey>,
    ) -> Result<CustodianScript, CoreError> {
        let mut builder = script::Builder::new();

        // Initial key check
        if let Some(initial_key) = initial_key {
            builder = builder
                .push_x_only_key(initial_key)
                .push_opcode(OP_CHECKSIGVERIFY);
        }

        // Sort covenant public keys
        let mut sorted_pks = custodian_pub_keys.to_owned();
        sorted_pks.sort();

        // Check for duplicates
        for i in 0..sorted_pks.len() - 1 {
            if sorted_pks[i] == sorted_pks[i + 1] {
                return Err(CoreError::DuplicateCustodianKeys);
            }
        }

        // Add covenant keys to the script
        builder = builder.push_x_only_key(&sorted_pks[0]);
        builder = builder.push_opcode(OP_CHECKSIG);

        for pk in sorted_pks.iter().skip(1) {
            builder = builder.push_x_only_key(pk);
            builder = builder.push_opcode(OP_CHECKSIGADD);
        }

        // Add quorum check
        builder = builder
            .push_int(custodian_quorum as i64)
            .push_opcode(OP_GREATERTHANOREQUAL);

        Ok(builder.into_script())
    }
}

impl BuildTwoPartyBranch for TwoPartyBranch {
    fn build(x: &XOnlyPublicKey, y: &XOnlyPublicKey) -> Result<Self, CoreError> {
        Ok(script::Builder::new()
            .push_x_only_key(x)
            .push_opcode(OP_CHECKSIGVERIFY)
            .push_x_only_key(y)
            .push_opcode(OP_CHECKSIG)
            .into_script())
    }
}

impl BuildCustodianAndPartyBranch for CustodianAndPartyBranch {
    fn build(
        x: &XOnlyPublicKey,
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
    ) -> Result<Self, CoreError> {
        let script = CustodianScriptBuilder::build(custodian_pub_keys, custodian_quorum, Some(x))?;
        Ok(script)
    }
}

impl BuildCustodianOnlyBranch for CustodianOnlyBranch {
    fn build(
        custodian_pub_keys: &[XOnlyPublicKey],
        custodian_quorum: u8,
    ) -> Result<Self, CoreError> {
        let script = CustodianScriptBuilder::build(custodian_pub_keys, custodian_quorum, None)?;
        Ok(script)
    }
}
