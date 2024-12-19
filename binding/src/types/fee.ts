export enum BTCFeeOpts {
  MinimumFee = 0,
  EconomyFee = 1,
  HourFee = 2,
  HalfHourFee = 3,
  FastestFee = 4,
}

export namespace BTCFeeOpts {
  export const Bytes = (feeOpts: BTCFeeOpts): Uint8Array => {
    return new Uint8Array([feeOpts]);
  };

  export const BytesString = (feeOpts: BTCFeeOpts): `0x${string}` => {
    const bytes = Bytes(feeOpts);
    const hex = Array.from(bytes)
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
    return `0x${hex}`;
  };
}
