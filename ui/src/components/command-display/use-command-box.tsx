import { Copy, SquareTerminal } from 'lucide-react';
import { Button } from '../ui/button';
import { Textarea } from '../ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '@/hooks/use-toast';
import { useSettings } from '@/use-command';

interface UseCommandBoxProps {
  command: string;
  commandId: string;
  onChangeCommand: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
}

export function UseCommandBox({
  command,
  commandId,
  onChangeCommand,
}: UseCommandBoxProps) {
  const [settings] = useSettings();

  function onUseCommand() {
    invoke('update_command_last_used', {
      commandId,
    }).catch((error) => {
      console.error(error);
      toast({
        title: `An error occurred whilst updating metadata. Please refer to logs. ❌`,
      });
    });
  }

  function onCopy() {
    navigator.clipboard.writeText(command);
    toast({
      title: 'Copied to clipboard ✅',
    });

    onUseCommand();
  }

  function onExecuteInTerminal() {
    invoke('execute_in_terminal', {
      command,
    })
      .then(() => {
        onUseCommand();
      })
      .catch((error) => {
        console.error(error);
        toast({
          title: `${error} ❌`,
        });
      });
  }

  return (
    <div className="relative w-full">
      <Textarea
        className="min-h-0 max-h-[76px] py-[7px] pr-16 bg-accent font-robotomono shadow resize-none"
        ref={(textarea) => {
          if (textarea) {
            textarea.style.height = '0px';
            textarea.style.height = textarea.scrollHeight + 2 + 'px';
          }
        }}
        value={command}
        onChange={onChangeCommand}
        autoCapitalize="off"
        autoCorrect="off"
      />
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon"
            type="button"
            onClick={onCopy}
            className="absolute right-0 top-0 m-2.5 h-4 w-4"
          >
            <Copy />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Copy command</TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon"
            type="button"
            onClick={onExecuteInTerminal}
            className="absolute right-8 top-0 m-2.5 h-4 w-4"
          >
            <SquareTerminal size={16} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Execute in {settings.default_terminal}</TooltipContent>
      </Tooltip>
    </div>
  );
}
