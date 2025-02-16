import React, { useEffect, useState } from 'react';
import { Box, Typography, Alert } from '@mui/material';
import { styled } from '@mui/material/styles';
import PubSub from 'pubsub-js';
import BitWidthToggle from './BitWidthToggle';

const ResultBox = styled(Box)(({ theme }) => ({
  marginTop: theme.spacing(2),
  padding: theme.spacing(2),
  border: `1px solid ${theme.palette.divider}`,
  borderRadius: 10,
  maxWidth: '600px',
  minWidth: '440px',
  minHeight: '200px',
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
  decimal: number | null;
  // original unsignedDecimal computed using minimal bitWidth two's complement
  unsignedDecimal: number | null;
  binary: string | null;
  octal: string | null;
  hexadecimal: string | null;
  minimalBitWidth: number;
}

// parseNumber now only supports hex formats ("0x" and "-0x")
const parseNumber = (result: string): ParsedResult => {
  console.log(result);
  let isNegative = false;
  let processedResult = result;

  // Check for and remove the negative sign
  if (processedResult.startsWith('-')) {
    isNegative = true;
    processedResult = processedResult.substring(1);
  }
  // Ensure result begins with "0x"
  if (!processedResult.startsWith('0x')) {
    throw new Error('Unexpected number format, expected hex prefix');
  }
  // Remove "0x" prefix and any underscores for readability
  const hexStr = processedResult.substring(2).replace(/_/g, '');
  const value = parseInt(hexStr, 16);

  // Determine the signed decimal.
  // If there is a negative prefix, simply apply it.
  // Otherwise, check if the hex number can be interpreted as negative
  // based on the candidate bit width determined by the hexStr length.
  let decimal: number;
  if (isNegative) {
    decimal = -value;
  } else {
    // Determine candidate bit width from hexStr length.
    let candidateBitWidth;
    if (hexStr.length <= 2) candidateBitWidth = 8;
    else if (hexStr.length <= 4) candidateBitWidth = 16;
    else if (hexStr.length <= 8) candidateBitWidth = 32;
    else candidateBitWidth = 64;

    // Calculate the half point for two's complement.
    const half = Math.pow(2, candidateBitWidth - 1);
    // If value is greater than or equal to half, it should be interpreted as negative.
    if (value >= half) {
      decimal = value - Math.pow(2, candidateBitWidth);
    } else {
      decimal = value;
    }
  }

  // Determine the minimal bit width required based on the interpreted decimal.
  const minimalBitWidth = getMinimalBitWidth(decimal);

  // Compute the unsigned decimal using two's complement based on minimalBitWidth.
  let unsignedDecimal: number;
  if (decimal < 0) {
    if (minimalBitWidth === 64) {
      unsignedDecimal = Number(BigInt(decimal) + (BigInt(1) << BigInt(64)));
    } else {
      unsignedDecimal = decimal + Math.pow(2, minimalBitWidth);
    }
  } else {
    unsignedDecimal = decimal;
  }

  // For display, initially use the minimal bit width representation.
  const display = recalcRepresentation(decimal, minimalBitWidth);

  return {
    decimal,
    unsignedDecimal,
    binary: display.binary,
    octal: display.octal,
    hexadecimal: display.hexadecimal,
    minimalBitWidth,
  };
};

// Helper function to compute minimal bitWidth for given signed number
const getMinimalBitWidth = (decimal: number): number => {
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

// Recalculate representations based on given bitWidth (user-selected)
const recalcRepresentation = (decimal: number, bitWidth: number) => {
  // For 64-bit negative numbers, use BigInt calculations to avoid precision loss.
  if (bitWidth === 64 && decimal < 0) {
    const unsignedBigInt = BigInt(decimal) + (BigInt(1) << BigInt(64));
    const binary = unsignedBigInt.toString(2).padStart(64, '0');
    const hexLength = Math.ceil(bitWidth / 4);
    const hexadecimal = unsignedBigInt.toString(16).toUpperCase().padStart(hexLength, '0');
    const octLength = Math.ceil(bitWidth / 3);
    const octal = unsignedBigInt.toString(8).padStart(octLength, '0');
    return { unsignedDecimal: unsignedBigInt.toString(), binary, octal, hexadecimal };
  }

  let unsignedDecimal: number;
  if (decimal < 0) {
    unsignedDecimal = decimal + Math.pow(2, bitWidth);
  } else {
    unsignedDecimal = decimal;
  }

  let binary: string;
  let octal: string;
  let hexadecimal: string;

  if (decimal < 0) {
    // Using BigInt for proper handling and padding of negative numbers (non-64bit)
    const unsignedBigInt = BigInt(unsignedDecimal);
    binary = unsignedBigInt.toString(2).padStart(bitWidth, '0');
    const hexLength = Math.ceil(bitWidth / 4);
    hexadecimal = unsignedBigInt.toString(16).toUpperCase().padStart(hexLength, '0');
    const octLength = Math.ceil(bitWidth / 3);
    octal = unsignedBigInt.toString(8).padStart(octLength, '0');
  } else {
    // For positive numbers, pad binary, hex, and octal based on the selected bitWidth.
    binary = decimal.toString(2).padStart(bitWidth, '0');
    const hexLength = Math.ceil(bitWidth / 4);
    hexadecimal = decimal.toString(16).toUpperCase().padStart(hexLength, '0');
    const octLength = Math.ceil(bitWidth / 3);
    octal = decimal.toString(8).padStart(octLength, '0');
  }

  return { unsignedDecimal, binary, octal, hexadecimal };
};

const ResultDisplay: React.FC = () => {
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [parsedResult, setParsedResult] = useState<ParsedResult | null>(null);
  // user-selected bit width for displaying the result
  const [selectedBitWidth, setSelectedBitWidth] = useState<number>(64);

  useEffect(() => {
    const token = PubSub.subscribe('cal_result', (_, data) => {
      setResult(data.result);
      setError(data.error);
    });

    // Cleanup subscription on unmount
    return () => {
      PubSub.unsubscribe(token);
    };
  }, []);

  // When result changes, parse it and set default selectedBitWidth to minimalBitWidth
  useEffect(() => {
    if (result) {
      try {
        const parsed = parseNumber(result);
        setParsedResult(parsed);
        setSelectedBitWidth(parsed.minimalBitWidth);
      } catch (err) {
        console.error('Parsing error:', err);
        setError((err as Error).message);
      }
    }
  }, [result]);

  // Recalculate displayed representations when selectedBitWidth changes
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
      {result && parsedResult && (
        <>
          {/* BitWidthToggle control */}
          <BitWidthToggle
            selectedBitWidth={selectedBitWidth}
            minimalBitWidth={parsedResult.minimalBitWidth}
            onChange={(newWidth) => setSelectedBitWidth(newWidth)}
          />
          {/* Display the results based on selected bit width */}
          <Typography variant="body1" color="primary">
            Binary: {displayRepresentation.binary}
          </Typography>
          <Typography variant="body1" color="secondary">
            Octal: {displayRepresentation.octal}
          </Typography>
          <Typography variant="body1" color="textPrimary">
            Signed Decimal: {parsedResult.decimal}
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