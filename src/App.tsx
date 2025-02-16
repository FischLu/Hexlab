import "./App.css";
import Header from './component/Header'
import ResultDisplay from './component/ResultDisplay'; 
import BinaryUI from './component/BinaryUI';
import { Box } from '@mui/material';

function App() {
  return (
    <main className="container">
      <h1>Cork calculator</h1>
      <Box mb={2}>
        {/* Header with bottom margin */}
        <Header />
      </Box>
      {/* <BinaryUI /> */}
      <Box mt={2}>
        {/* ResultDisplay with top margin */}
        <ResultDisplay />
      </Box>
    </main>
  );
}

export default App;
