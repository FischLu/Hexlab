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

  return (
    <main className="container">
      {/* <h1>Cork calculator</h1> */}
      <Box mb={2}>
        <Header />
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
        />
      </Box>
      <Box>
        {/* Pass binaryValue to ResultDisplay */}
        <ResultDisplay 
          selectedBitWidth={selectedBitWidth} 
          onMinimalBitWidthChange={setMinimalBitWidth}
        />
      </Box>
    </main>
  );
}

export default App;