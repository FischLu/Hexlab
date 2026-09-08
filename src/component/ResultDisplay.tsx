import React, { useEffect, useState } from 'react';
import { Box, Typography, Alert } from '@mui/material';
import { styled } from '@mui/material/styles';
import PubSub from 'pubsub-js';
import { CalculateResultMessage } from '../types';

const ResultBox = styled(Box)(({ theme }) => ({
  marginTop: theme.spacing(2),
  padding: theme.spacing(2),
  border: `1px solid ${theme.palette.divider}`,
  borderRadius: 10,
  maxWidth: '600px',
  minWidth: '440px',
  height: '150px',
  margin: '0 auto',
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center'
}));

const ErrorMessage = styled(Typography)(() => ({
  whiteSpace: 'pre-wrap',
  fontFamily: 'monospace',
  margin: 0,
  textAlign: 'left',
  display: 'block',
  fontSize: '0.75rem',
  overflowWrap: 'break-word',
  wordBreak: 'break-word',
  maxWidth: '100%'
}));

interface ParsedResult {
  decimal: bigint | null;
  // original unsignedDecimal computed using minimal bitWidth two's complement
  unsignedDecimal: number | null;
  binary: string | null;
  octal: string | null;
  hexadecimal: string | null;
}

// Recalculate representations based on given bitWidth (user-selected)
const recalcRepresentation = (decimal: bigint, bitWidth: number) => {
  const bitWidthBig = BigInt(bitWidth);
  const modulo = 1n << bitWidthBig;
  // Truncate to lower bitWidth bits to preserve bit pattern when switching widths
  const unsignedValue = ((decimal % modulo) + modulo) % modulo;
  // Reinterpret as signed for current width (e.g. 0xFFFD6D64 in 32 bits -> -168604)
  let signedValue = unsignedValue;
  if (bitWidth < 64) {
    const signBit = 1n << BigInt(bitWidth - 1);
    if (unsignedValue >= signBit) {
      signedValue = unsignedValue - modulo;
    }
  } else {
    // 64-bit: values come from i64, so unsigned >= 2^63 would mean negative
    const signBit64 = 1n << 63n;
    if (unsignedValue >= signBit64 && decimal < 0n) {
      signedValue = unsignedValue - modulo;
    }
  }

  const binary = unsignedValue.toString(2).padStart(Number(bitWidth), '0');
  const hexLength = Math.ceil(Number(bitWidth) / 4);
  const hexadecimal = unsignedValue.toString(16).toUpperCase().padStart(hexLength, '0');
  const octLength = Math.ceil(Number(bitWidth) / 3);
  const octal = unsignedValue.toString(8).padStart(octLength, '0');

  return {
    signedValue,
    unsignedDecimal: Number(unsignedValue),
    binary,
    octal,
    hexadecimal
  };
};

const ResultDisplay: React.FC = () => {
  const [error, setError] = useState<string | null>(null);
  const [parsedResult, setParsedResult] = useState<ParsedResult | null>(null);
  const [selectedBitWidth, setSelectedBitWidth] = useState<number>(64);

  useEffect(() => {
    const token = PubSub.subscribe('CALCULATE_RESULT', (_msg: string, data: CalculateResultMessage) => {
      const { bigIntResult, error, bitWidth } = data;
      if (!error && bigIntResult !== null) {
        try {
          const display = recalcRepresentation(bigIntResult, bitWidth);
          setParsedResult({
            decimal: bigIntResult,
            unsignedDecimal: display.unsignedDecimal,
            binary: display.binary,
            octal: display.octal,
            hexadecimal: display.hexadecimal,
          });
          setSelectedBitWidth(bitWidth);
          setError(null);
        } catch (err) {
          setError((err as Error).message);
          setParsedResult(null);
        }
      } else {
        setError(error);
        setParsedResult(null);
      }
    });

    return () => {
      PubSub.unsubscribe(token);
    };
  }, []);

  const displayRepresentation = parsedResult
    ? recalcRepresentation(parsedResult.decimal!, selectedBitWidth)
    : { binary: '', octal: '', hexadecimal: '', unsignedDecimal: 0, signedValue: 0n };

  return (
    <ResultBox>
      {error && (
        <Alert severity="error">
          <ErrorMessage className="selectable-text" as="pre" variant="body2">
            {error}
          </ErrorMessage>
        </Alert>
      )}
      {parsedResult && (
        <>
          <Typography className="selectable-text" variant="body1" color="primary">
            Binary: {displayRepresentation.binary}
          </Typography>
          <Typography className="selectable-text" variant="body1" color="secondary">
            Octal: {displayRepresentation.octal}
          </Typography>
          <Typography className="selectable-text" variant="body1" color="textPrimary">
            Signed Decimal: {displayRepresentation.signedValue.toString()}
          </Typography>
          <Typography className="selectable-text" variant="body1" color="textSecondary">
            Unsigned Decimal: {displayRepresentation.unsignedDecimal}
          </Typography>
          <Typography className="selectable-text" variant="body1" color="error">
            Hexadecimal: {displayRepresentation.hexadecimal}
          </Typography>
        </>
      )}
    </ResultBox>
  );
};

export default ResultDisplay;