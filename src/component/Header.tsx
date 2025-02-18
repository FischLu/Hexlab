import { useState, useEffect } from "react";
import PubSub from 'pubsub-js';
import { invoke } from "@tauri-apps/api/core";
import { Box, Button, TextField, ToggleButtonGroup, ToggleButton } from '@mui/material';
import { styled } from '@mui/material/styles';
import { CalculateResultMessage } from '../types';

// Container for the header elements
const Container = styled(Box)(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: theme.spacing(2), // Modified: Changed from theme.spacing(2) to theme.spacing(1)
  maxWidth: "430px",
  margin: "0 auto",
  minWidth: "330px",
  paddingTop: theme.spacing(0), // Add a small top padding if needed
}));

// Styled toggle button to be reused across components
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
}));

// Styled TextField with custom scrollbar styling
const StyledTextField = styled(TextField)(() => ({
  width: '350px',
  '& .MuiInputBase-input': {
    maxHeight: '150px',
    overflowY: 'scroll',
    scrollbarWidth: 'thin', // Firefox
    scrollbarColor: '#888 transparent', // Firefox
    '&::-webkit-scrollbar': {
      width: '8px'
    },
    '&::-webkit-scrollbar-thumb': {
      backgroundColor: '#888',
      borderRadius: '4px'
    },
    '&::-webkit-scrollbar-track': {
      backgroundColor: 'transparent',
    }
  }
}));

export default function Header() {
  const [expression, setExpression] = useState('');
  const [mode, setMode] = useState('hex'); // 'dec' or 'hex'

  const handleEvaluate = async () => {
    try {
      const res: string = await invoke('evaluate_expression', { 
        exprStr: expression,
        options: { mode }
      });
      // Process hexadecimal strings with a negative sign
      let isNegative = false;
      let processedRes = res;
      if (processedRes.startsWith('-')) {
        isNegative = true;
        processedRes = processedRes.substring(1);
      }
      
       // Create a BigInt using the processed string
      const value = BigInt(processedRes);
      // If the original string is negative, convert value to a negative number
      const finalValue = isNegative ? -value : value;
      // console.log("Header:", finalValue)
      const message: CalculateResultMessage = { hexResult: finalValue, error: null }
      PubSub.publish('CALCULATE_RESULT', message);
    } catch (err) {
      const message: CalculateResultMessage = { hexResult: null, error: `Error: ${err}` }
      PubSub.publish('CALCULATE_RESULT', message);
    }
  };

  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter') {
      event.preventDefault();
      handleEvaluate();
    }
  };

  const handleModeChange = (_: React.MouseEvent<HTMLElement>, newMode: string) => {
    if (newMode !== null) {
      setMode(newMode);
    }
  };

  // useEffect to trigger evaluation whenever mode changes
  useEffect(() => {
    if (expression.trim() !== '') {
      handleEvaluate();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mode]);

  return (
    <Container>
      <ToggleButtonGroup
        value={mode}
        exclusive
        onChange={handleModeChange}
        aria-label="calculation mode"
        size="small"
        sx={{ alignSelf: 'center' }}
      >
        <StyledToggleButton value="hex" aria-label="hexadecimal mode">
          Hex mode
        </StyledToggleButton>
        <StyledToggleButton value="dec" aria-label="decimal mode">
          Dec mode
        </StyledToggleButton>
      </ToggleButtonGroup>

      <Box display="flex" alignItems="center" gap={2}>
        <StyledTextField
          id="expression-input"
          label="Enter an expression"
          variant="outlined"
          multiline
          maxRows={4}
          minRows={1}
          onChange={(e) => setExpression(e.currentTarget.value)}
          onKeyDown={handleKeyPress}
          slotProps={{
            inputLabel: {
              shrink: true
            }
          }}
        />
        <Button variant="contained" color="primary" onClick={handleEvaluate}>
          Calc
        </Button>
      </Box>
    </Container>
  )
}