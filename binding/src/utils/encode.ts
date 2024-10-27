export const hexToBytes = (hex: string) => {
  return new Uint8Array(Buffer.from(hex, "hex"));
};

export const bytesToHex = (buffer: Uint8Array) => {
  return Buffer.from(buffer).toString('hex');
};
