import "./App.css";
import Header from './component/Header';
import ResultDisplay from './component/ResultDisplay'; 
import BinaryUI from './component/BinaryUI';
import BitWidthToggle from './component/BitWidthToggle';
import { Box } from '@mui/material';
import { useState } from 'react';

function App() {
  const [selectedBitWidth, setSelectedBitWidth] = useState<number>(64);
  const [minimalBitWidth, setMinimalBitWidth] = useState<number>(8);
  const [binaryValue, setBinaryValue] = useState<bigint>(0n);

  // Central onCalculate callback, updates binaryValue based on Header computation
  const handleCalculateResult = (hexResult: string, error: string | null) => {
    if (!error && hexResult) {
      try {
        const newVal = BigInt(hexResult);
        setBinaryValue(newVal);
      } catch (err) {
        console.error("Failed to convert to BigInt", err);
      }
    }
  };

  return (
    <main className="container">
      {/* <h1>Cork calculator</h1> */}
      <Box mb={2}>
        <Header onCalculate={handleCalculateResult} />
      </Box>
      <Box mb={2}>
        <BitWidthToggle
          selectedBitWidth={selectedBitWidth}
          minimalBitWidth={minimalBitWidth}
          onChange={setSelectedBitWidth}
        />
      </Box>
      <Box mb={2}>
        <BinaryUI 
          selectedBitWidth={selectedBitWidth} 
          value={binaryValue}
          onChange={setBinaryValue}
        />
      </Box>
      <Box>
        {/* Pass binaryValue to ResultDisplay */}
        <ResultDisplay 
          selectedBitWidth={selectedBitWidth} 
          binaryValue={binaryValue}
          onMinimalBitWidthChange={setMinimalBitWidth}
        />
      </Box>
    </main>
  );
}

export default App;