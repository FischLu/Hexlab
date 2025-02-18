import React from 'react';
import { Box, ToggleButton } from '@mui/material';
import { styled } from '@mui/material/styles';

// Styled container for the binary display
const BinaryContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: theme.spacing(2),
  maxWidth: '600px', // Modified: Changed from 800px to 600px
  margin: '0 auto',
  padding: theme.spacing(1), // Modified: Changed from theme.spacing(2) to theme.spacing(1)
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
  gap: 0, // Modified: Directly connected with no gap (originally theme.spacing(0.5))
  marginRight: theme.spacing(0),
  '&:last-child': {
    marginRight: 0,
  },
}));

// Styled bit button
const BitButton = styled(ToggleButton)(({ theme }) => ({
  width: '36px',
  height: '36px',
  padding: 0,
  border: `1px solid ${theme.palette.divider}`,
  borderRadius: 0,
  '&.Mui-disabled': {
    opacity: 0.5,
    backgroundColor: theme.palette.action.disabledBackground,
  },
  '&.Mui-selected': {
    backgroundColor: theme.palette.primary.light,
    color: theme.palette.primary.contrastText,
    '&:hover': {
      backgroundColor: theme.palette.primary.main,
    },
  },
}));

// Styled bit index label
const BitIndex = styled(Box)({
  fontSize: '0.75rem',
  textAlign: 'center',
  marginTop: '4px',
});

export interface BinaryUIProps {
  selectedBitWidth: number;
  value: bigint;
  onChange: (value: bigint) => void;
}

const BinaryUI: React.FC<BinaryUIProps> = ({ selectedBitWidth, value = 0n, onChange }) => {
  // Handle bit toggle, removed PubSub.publish call
  const handleBitToggle = (bitPosition: number) => {
    const mask = 1n << BigInt(bitPosition);
    const newValue = value ^ mask;
    onChange(newValue);
  };

  // Updated getBit implementation using division and modulo to avoid implicit BigInt to number conversion
  const getBit = (position: number): boolean => {
    return (value / (1n << BigInt(position)) % 2n) === 1n;
  };

  // Generate 64 bit positions in descending order and split into 4 rows of 16 bits each.
  const allBits = Array.from({ length: 64 }, (_, i) => i).reverse();
  const rows = [];
  for (let i = 0; i < 64; i += 16) {
    rows.push(allBits.slice(i, i + 16));
  }

  return (
    <BinaryContainer>
      {rows.map((row, rowIdx) => {
        // Divide each row into two groups, each group containing 8 bits
        const groups = [row.slice(0, 8), row.slice(8, 16)];
        return (
          <BitRow key={rowIdx} sx={{ gap: 2 }}> {/* Set the gap between groups */}
            {groups.map((group, groupIdx) => (
              <BitGroup key={groupIdx}>
                {group.map((position) => (
                  <Box key={position}>
                    <BitButton
                      value={position.toString()}
                      selected={getBit(position)}
                      onChange={() => handleBitToggle(position)}
                      disabled={position >= selectedBitWidth}
                    >
                      {getBit(position) ? '1' : '0'}
                    </BitButton>
                    <BitIndex>{position}</BitIndex>
                  </Box>
                ))}
              </BitGroup>
            ))}
          </BitRow>
        );
      })}
    </BinaryContainer>
  );
};

export default BinaryUI;