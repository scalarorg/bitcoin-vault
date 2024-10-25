import { ethers } from "ethers";
// const MNEMONIC: string = "test test test test test test test test test test test junk";
const MNEMONIC: string = "pride erupt add surge eagle once nerve guide good phrase argue alert budget welcome humor bullet slim safe mom review run dry decrease fox";
export function getDefaultEthAddress(): string {
    const mnemonic = ethers.Mnemonic.fromPhrase(MNEMONIC);
    const wallet = ethers.HDNodeWallet.fromMnemonic(mnemonic, "m/44'/60'/0'/0/0");
    return wallet.address;
}
