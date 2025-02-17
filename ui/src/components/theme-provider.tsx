import { Theme } from '@/types/config';
import { useSettings } from '@/use-command';
import { useEffect } from 'react';

type ThemeProviderProps = {
  children: React.ReactNode;
  defaultTheme?: Theme;
};

export function ThemeProvider({
  children,
  defaultTheme = Theme.System,
  ...props
}: ThemeProviderProps) {
  const [settings, _] = useSettings();

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    if (settings.theme === Theme.System) {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
        .matches
        ? 'dark'
        : 'light';

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(settings.theme.toLowerCase());
  }, [settings.theme]);

  return <>{children}</>;
}
