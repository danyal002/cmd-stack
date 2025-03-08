import { FolderDot, Moon, Sun } from 'lucide-react';

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
        console.log(res);
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
        <Button variant="outline" className="w-24">
          {settings.application_theme == ApplicationTheme.System && (
            <>
              <FolderDot />
              <div className="ml-auto">System</div>
            </>
          )}
          {settings.application_theme == ApplicationTheme.Light && (
            <>
              <Sun />
              <div className="ml-auto">Light</div>
            </>
          )}
          {settings.application_theme == ApplicationTheme.Dark && (
            <>
              <Moon />
              <div className="ml-auto">Dark</div>
            </>
          )}
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem
          onClick={() => setTheme(ApplicationTheme.System)}
          className="cursor-pointer hover:bg-accent"
        >
          <FolderDot />
          System
        </DropdownMenuItem>
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
