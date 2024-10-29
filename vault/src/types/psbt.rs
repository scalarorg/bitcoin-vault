pub struct VaultPSBT {
    pub tx: Transaction,
    pub inputs: Vec<PSBTInput>,
    pub outputs: Vec<PSBTOutput>,
}
impl VaultPSBT {
    pub fn new(tx: Transaction, inputs: Vec<PSBTInput>, outputs: Vec<PSBTOutput>) -> Self {
        Self {
            tx,
            inputs,
            outputs,
        }
    }
}
