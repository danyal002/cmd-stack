import React, { useEffect } from 'react';
import { useSystemTheme } from './useSystemTheme';

const ThemeManager: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const theme = useSystemTheme();

  useEffect(() => {
    const root = document.documentElement;
    // Remove any previous theme classes
    root.classList.remove('light', 'dark');
    // Add the current theme
    root.classList.add(theme);
  }, [theme]);

  return <>{children}</>;
};

export default ThemeManager;
