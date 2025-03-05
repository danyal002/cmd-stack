import { Copy, SquareTerminal } from 'lucide-react';
import { Button } from '../ui/button';
import { Textarea } from '../ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '@/hooks/use-toast';

interface UseCommandBoxProps {
  disabled: boolean;
  command: string;
  commandId: string;
  onChangeCommand: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
}

export function UseCommandBox({
  disabled,
  command,
  commandId,
  onChangeCommand,
}: UseCommandBoxProps) {
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
        disabled={disabled}
        className="min-h-0 max-h-[76px] py-[7px] pr-16 bg-accent font-spacemono shadow resize-none"
        ref={(textarea) => {
          if (textarea) {
            textarea.style.height = '0px';
            textarea.style.height = textarea.scrollHeight + 2 + 'px';
          }
        }}
        value={command}
        onChange={onChangeCommand}
      />
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon"
            type="button"
            disabled={disabled}
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
            disabled={disabled}
            onClick={onExecuteInTerminal}
            className="absolute right-8 top-0 m-2.5 h-4 w-4"
          >
            <SquareTerminal size={16} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Execute in terminal</TooltipContent>
      </Tooltip>
    </div>
  );
}
