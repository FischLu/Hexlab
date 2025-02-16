import React from 'react';
import { Box, ToggleButton } from '@mui/material';
import { styled } from '@mui/material/styles';
import PubSub from 'pubsub-js';

// Styled container for the binary display
const BinaryContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: theme.spacing(2),
  maxWidth: '800px',
  margin: '0 auto',
  padding: theme.spacing(2),
}));

// Styled row for bits
const BitRow = styled(Box)(({ theme }) => ({
  display: 'flex',
  gap: theme.spacing(0.5),
  alignItems: 'flex-start',
  justifyContent: 'center',
}));

// Styled group for 8 bits
const BitGroup = styled(Box)(({ theme }) => ({
  display: 'flex',
  gap: theme.spacing(0.5),
  marginRight: theme.spacing(2),
  '&:last-child': {
    marginRight: 0,
  },
}));

// Styled bit button
const BitButton = styled(ToggleButton)(({ theme }) => ({
  width: '32px',
  height: '32px',
  padding: 0,
  border: `1px solid ${theme.palette.divider}`,
  '&.Mui-disabled': {
    opacity: 0.5,
    backgroundColor: theme.palette.action.disabledBackground,
  },
  '&.Mui-selected': {
    backgroundColor: theme.palette.primary.main,
    color: theme.palette.primary.contrastText,
    '&:hover': {
      backgroundColor: theme.palette.primary.dark,
    },
  },
}));

// Styled bit index label
const BitIndex = styled(Box)({
  fontSize: '0.75rem',
  textAlign: 'center',
  marginTop: '4px',
});

interface BinaryUIProps {
  selectedBitWidth: number;
  value: bigint;
}

const BinaryUI: React.FC<BinaryUIProps> = ({ selectedBitWidth, value }) => {
  // Handle bit toggle
  const handleBitToggle = (bitPosition: number) => {
    const newValue = value ^ (1n << BigInt(bitPosition));
    PubSub.publish('VALUE_CHANGED', { value: newValue });
  };

  // Create array of bit positions
  const upperBits = Array.from({ length: 32 }, (_, i) => i + 32).reverse();
  const lowerBits = Array.from({ length: 32 }, (_, i) => i).reverse();

  // Helper function to render bit groups
  const renderBitGroup = (bits: number[], disabled: boolean) => {
    return bits.reduce((acc: JSX.Element[], bit, index) => {
      if (index % 8 === 0) {
        acc.push(
          <BitGroup key={`group-${bit}`}>
            {bits.slice(index, index + 8).map((position) => (
              <Box key={position}>
                <BitButton
                  value={position}
                  selected={!!(value & (1n << BigInt(position)))}
                  onChange={() => handleBitToggle(position)}
                  disabled={disabled || position >= selectedBitWidth}
                >
                  {!!(value & (1n << BigInt(position))) ? '1' : '0'}
                </BitButton>
                <BitIndex>{position}</BitIndex>
              </Box>
            ))}
          </BitGroup>
        );
      }
      return acc;
    }, []);
  };

  return (
    <BinaryContainer>
      <BitRow>
        {renderBitGroup(upperBits, false)}
      </BitRow>
      <BitRow>
        {renderBitGroup(lowerBits, false)}
      </BitRow>
    </BinaryContainer>
  );
};

export default BinaryUI;