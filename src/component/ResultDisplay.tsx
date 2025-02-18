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
  minHeight: '130px',
  margin: '0 auto'
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
  // minimalBitWidth: number;
}

// Recalculate representations based on given bitWidth (user-selected)
const recalcRepresentation = (decimal: bigint, bitWidth: number) => {
  const bitWidthBig = BigInt(bitWidth);
  const unsignedValue = decimal < 0n ? 
    decimal + (1n << bitWidthBig) : 
    decimal;

  const binary = unsignedValue.toString(2).padStart(Number(bitWidth), '0');
  const hexLength = Math.ceil(Number(bitWidth) / 4);
  const hexadecimal = unsignedValue.toString(16).toUpperCase().padStart(hexLength, '0');
  const octLength = Math.ceil(Number(bitWidth) / 3);
  const octal = unsignedValue.toString(8).padStart(octLength, '0');

  return {
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
    : { binary: '', octal: '', hexadecimal: '', unsignedDecimal: 0 };

  return (
    <ResultBox>
      {error && (
        <Alert severity="error">
          <ErrorMessage as="pre" variant="body2">
            {error}
          </ErrorMessage>
        </Alert>
      )}
      {parsedResult && (
        <>
          <Typography variant="body1" color="primary">
            Binary: {displayRepresentation.binary}
          </Typography>
          <Typography variant="body1" color="secondary">
            Octal: {displayRepresentation.octal}
          </Typography>
          <Typography variant="body1" color="textPrimary">
            Signed Decimal: {parsedResult.decimal?.toString()}
          </Typography>
          <Typography variant="body1" color="textSecondary">
            Unsigned Decimal: {displayRepresentation.unsignedDecimal}
          </Typography>
          <Typography variant="body1" color="error">
            Hexadecimal: {displayRepresentation.hexadecimal}
          </Typography>
        </>
      )}
    </ResultBox>
  );
};

export default ResultDisplay;