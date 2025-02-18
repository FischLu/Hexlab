import React, { useEffect } from 'react';
import { ToggleButtonGroup, ToggleButton } from '@mui/material';
import { styled } from '@mui/material/styles';

// Reuse the same style as in Header
const StyledToggleButton = styled(ToggleButton)(({ theme }) => ({
  '&.MuiToggleButton-root': {
    textTransform: 'none',
    padding: '4px 12px',
    color: theme.palette.text.secondary,
    borderColor: theme.palette.divider,
  },
  '&.Mui-selected': {
    color: theme.palette.primary.main,
    backgroundColor: theme.palette.action.selected,
    '&:hover': {
      backgroundColor: theme.palette.action.hover,
    },
  },
  // Improve the disabled state appearance
  '&.Mui-disabled': {
    opacity: 0.5, // Lower opacity makes it visually distinct
    color: theme.palette.text.disabled,
    borderColor: theme.palette.action.disabled,
  }
}));

interface BitWidthToggleProps {
  selectedBitWidth: number;
  minimalBitWidth: number;
  onChange: (newBitWidth: number) => void;
}

const BitWidthToggle: React.FC<BitWidthToggleProps> = ({ selectedBitWidth, minimalBitWidth, onChange }) => {
  
  // Automatically update selectedBitWidth if it falls below minimalBitWidth
  useEffect(() => {
    if (selectedBitWidth < minimalBitWidth) {
      onChange(minimalBitWidth);
    }
  }, [selectedBitWidth, minimalBitWidth, onChange]);

  const handleChange = (_: React.MouseEvent<HTMLElement>, newValue: number | null) => {
    if (newValue !== null) {
      onChange(newValue);
    }
  };

  // Available options: 8, 16, 32, 64 bits.
  const options = [8, 16, 32, 64];

  return (
    <ToggleButtonGroup
      value={selectedBitWidth}
      exclusive
      onChange={handleChange}
      aria-label="bit width selection"
      size="small"
      sx={{ alignSelf: 'center', mb: 2 }}
    >
      {options.map((width) => (
        <StyledToggleButton key={width} value={width} disabled={width < minimalBitWidth}>
          {width}bit
        </StyledToggleButton>
      ))}
    </ToggleButtonGroup>
  );
};

export default BitWidthToggle;