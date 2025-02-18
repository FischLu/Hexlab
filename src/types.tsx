export interface CalculateResultMessage {
  bigIntResult: bigint | null;
  bitWidth: number;
  error: string | null;
}
