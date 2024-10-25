// use bitcoin::{
//     key::Secp256k1,
//     psbt::{GetKey, KeyRequest},
//     secp256k1::Signing,
//     PrivateKey,
// };

// pub struct SinglePrivKey(PrivateKey);

// impl SinglePrivKey {
//     pub fn new(privkey: PrivateKey) -> Self {
//         SinglePrivKey(privkey)
//     }
// }

// impl GetKey for SinglePrivKey {
//     type Error = &'static str;

//     fn get_key<C: Signing>(
//         &self,
//         key_request: KeyRequest,
//         secp: &Secp256k1<C>,
//     ) -> Result<Option<PrivateKey>, Self::Error> {
//         match key_request {
//             KeyRequest::Pubkey(pubkey) => {
//                 if pubkey == self.0.public_key(secp) {
//                     Ok(Some(self.0))
//                 } else {
//                     Ok(None)
//                 }
//             }
//             KeyRequest::Bip32(_) => Ok(None), // We don't support BIP32 derivation for a single key
//             _ => Err("Unsupported key request"),
//         }
//     }
// }
