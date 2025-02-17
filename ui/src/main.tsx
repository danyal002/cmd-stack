import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import ThemeManager from '@/components/theme-manager';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeManager>
      <App />
    </ThemeManager>
  </React.StrictMode>,
);
