import { DollarSign, SquareTerminal } from 'lucide-react';

import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { toast } from '@/hooks/use-toast';
import { DefaultTerminal } from '@/types/config';
import { useSettings } from '@/use-command';
import { invoke } from '@tauri-apps/api/core';

export function TerminalToggle() {
  const [settings, refreshSettings] = useSettings();

  function setTerminal(terminal: DefaultTerminal) {
    settings.default_terminal = terminal;

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
          {settings.default_terminal == DefaultTerminal.Terminal && (
            <>
              <SquareTerminal />
              <div className="ml-auto">Terminal</div>
            </>
          )}
          {settings.default_terminal == DefaultTerminal.Iterm && (
            <>
              <DollarSign />
              <div className="ml-auto">iTerm</div>
            </>
          )}
          <span className="sr-only">Toggle default terminal</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem
          onClick={() => setTerminal(DefaultTerminal.Terminal)}
          className="cursor-pointer hover:bg-accent"
        >
          <SquareTerminal />
          Terminal
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setTerminal(DefaultTerminal.Iterm)}
          className="cursor-pointer hover:bg-accent"
        >
          <DollarSign />
          iTerm
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
