import { FixedSizeBinary } from "polkadot-api";

export function bigintToFixedSizeArray4(
  value: bigint
): [bigint, bigint, bigint, bigint] {
  const mask = (1n << 64n) - 1n;

  const part0 = value & mask;
  const part1 = (value >> 64n) & mask;
  const part2 = (value >> 128n) & mask;
  const part3 = (value >> 192n) & mask;

  return [part0, part1, part2, part3];
}

export function fixedSizeArray4ToBigint(
  arr: [bigint, bigint, bigint, bigint]
): bigint {
  return (
    (arr[0] & ((1n << 64n) - 1n)) |
    ((arr[1] & ((1n << 64n) - 1n)) << 64n) |
    ((arr[2] & ((1n << 64n) - 1n)) << 128n) |
    ((arr[3] & ((1n << 64n) - 1n)) << 192n)
  );
}

export const isHex = (str: string) => {
  return typeof str === 'string' && /^0x[0-9a-fA-F]+$/.test(str)
}


export function accountToHex(address: string, isPrefixed = true) {
  if (isHex(address)) return address;
  const hex = FixedSizeBinary.fromAccountId32<32>(address).asHex();
  return isPrefixed ? hex : hex.slice(2);
}