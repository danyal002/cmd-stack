import { Moon, Sun } from 'lucide-react';

import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { toast } from '@/hooks/use-toast';
import { ApplicationTheme } from '@/types/config';
import { useSettings } from '@/use-command';
import { invoke } from '@tauri-apps/api/core';

export function ModeToggle() {
  const [settings, refreshSettings] = useSettings();

  function setTheme(application_theme: ApplicationTheme) {
    settings.application_theme = application_theme;

    invoke('write_config', { config: settings })
      .then((res) => {
        toast({
          title: 'Settings updated ✅ ',
        });
        refreshSettings();
      })
      .catch((error) => {
        console.log(error);
        toast({
          title: `${error} ❌`,
        });
      });
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon">
          <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem
          onClick={() => setTheme(ApplicationTheme.Light)}
          className="cursor-pointer hover:bg-accent"
        >
          <Sun />
          Light
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setTheme(ApplicationTheme.Dark)}
          className="cursor-pointer hover:bg-accent"
        >
          <Moon />
          Dark
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
