import "./App.css";
import Header from './component/Header';
import ResultDisplay from './component/ResultDisplay'; 
import BinaryUI from './component/BinaryUI';
import BitWidthToggle from './component/BitWidthToggle';
import { Box } from '@mui/material';

function App() {
  return (
    <main className="container">
      {/* <h1>Cork calculator</h1> */}
      <Box mb={2}>
        <Header />
      </Box>
      <Box mb={2}>
        <BitWidthToggle />
      </Box>
      <Box mb={2}>
        <BinaryUI />
      </Box>
      <Box>
        <ResultDisplay />
      </Box>
    </main>
  );
}

export default App;