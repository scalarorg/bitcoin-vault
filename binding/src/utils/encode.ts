export const hexToBytes = (hex: string) => {
  return new Uint8Array(Buffer.from(hex, "hex"));
};
