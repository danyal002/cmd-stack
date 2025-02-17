import { ApplicationTheme } from '@/types/config';
import { useSettings } from '@/use-command';
import { useEffect } from 'react';

type ThemeProviderProps = {
  children: React.ReactNode;
  defaultTheme?: ApplicationTheme;
};

export function ThemeProvider({
  children,
  defaultTheme = ApplicationTheme.System,
  ...props
}: ThemeProviderProps) {
  const [settings, _] = useSettings();

  function applyTheme() {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    if (settings.application_theme === ApplicationTheme.System) {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
        .matches
        ? 'dark'
        : 'light';

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(settings.application_theme.toLowerCase());
  }

  useEffect(() => {
    applyTheme();
  }, [settings.application_theme]);

  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    const handleChange = () => {
      applyTheme();
    };

    mediaQuery.addEventListener("change", handleChange);

    return () => mediaQuery.removeEventListener("change", handleChange);
  }, [settings.application_theme]);

  return <>{children}</>;
}
