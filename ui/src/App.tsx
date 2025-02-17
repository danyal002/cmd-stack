import { ThemeProvider } from './components/theme-provider';
import './index.css';
import CommandPage from './page';

function App() {
  return (
    <main>
      <ThemeProvider>
        <CommandPage />
      </ThemeProvider>
    </main>
  );
}

export default App;
