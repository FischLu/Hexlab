import React, { useEffect, useState } from 'react';
import { ToggleButtonGroup, ToggleButton } from '@mui/material';
import { styled } from '@mui/material/styles';
import { CalculateResultMessage } from '../types';
import { getMinimalBitWidth } from '../supportFunctions';

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

const BitWidthToggle: React.FC = () => {
  const [globalResult, setGlobalResult] = useState<bigint | null>(0n);
  const [hasError, setHasError] = useState<boolean>(false);
  const [minimalBitWidth, setMinimalBitWidth] = useState<number>(8);
  const [selectedBitWidth, setSelectedBitWidth] = useState<number>(64);

  useEffect(() => {
    const token = PubSub.subscribe('CALCULATE_RESULT', (_msg: string, data: CalculateResultMessage) => {
      const { bigIntResult, error, bitWidth } = data;
      if (!error && bigIntResult !== null) {
        setGlobalResult(bigIntResult);
        setHasError(false);
        setSelectedBitWidth(bitWidth);
        const newMinimalBitWidth = getMinimalBitWidth(bigIntResult);
        setMinimalBitWidth(newMinimalBitWidth);
      } else {
        setGlobalResult(null)
        setHasError(true);
      }
    });

    return () => {
      PubSub.unsubscribe(token);
    };
  }, []);

  const handleChange = (_event: React.MouseEvent<HTMLElement>, newValue: number | null) => {
    if (newValue !== null) {
      setSelectedBitWidth(newValue);
      const message: CalculateResultMessage = { bigIntResult: globalResult, error: null, bitWidth: newValue }
      PubSub.publish('CALCULATE_RESULT', message);
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
        <StyledToggleButton key={width} value={width} disabled={hasError || width < minimalBitWidth}>
          {width}bit
        </StyledToggleButton>
      ))}
    </ToggleButtonGroup>
  );
};

export default BitWidthToggle;