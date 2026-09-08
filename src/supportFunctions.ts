
// Helper function to compute minimal bitWidth for given value.
// For negative numbers, use signed ranges (two's complement must fit).
// For non-negative numbers, use unsigned ranges, so that e.g. 0xFFFD6D64
// (4294798692) only needs 32 bits and can still be viewed as signed 32-bit negative.
export const getMinimalBitWidth = (decimal: bigint): number => {
  if (decimal < 0) {
    if (decimal >= -128) {
      return 8;
    } else if (decimal >= -32768) {
      return 16;
    } else if (decimal >= -2147483648) {
      return 32;
    } else {
      return 64;
    }
  } else {
    if (decimal <= 0xFF) {
      return 8;
    } else if (decimal <= 0xFFFF) {
      return 16;
    } else if (decimal <= 0xFFFFFFFF) {
      return 32;
    } else {
      return 64;
    }
  }
};

// Reinterpret a stored value as signed integer for a given bit width.
// Positive values whose bit pattern has the sign bit set become negative,
// e.g. 0xFFFD6D64 (4294798692) in 32 bits -> -168604.
export const toSignedForWidth = (decimal: bigint, bitWidth: number): bigint => {
  if (decimal < 0n) {
    return decimal;
  }
  if (bitWidth >= 64) {
    return decimal;
  }
  const signBit = 1n << BigInt(bitWidth - 1);
  if (decimal >= signBit) {
    return decimal - (1n << BigInt(bitWidth));
  }
  return decimal;
};