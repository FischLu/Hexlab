
// Helper function to compute minimal bitWidth for given signed number
export const getMinimalBitWidth = (decimal: bigint): number => {
  if (decimal >= -128 && decimal <= 127) {
    return 8;
  } else if (decimal >= -32768 && decimal <= 32767) {
    return 16;
  } else if (decimal >= -2147483648 && decimal <= 2147483647) {
    return 32;
  } else {
    return 64;
  }
};