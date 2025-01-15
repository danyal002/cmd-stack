import format from 'date-fns/format';
import { Pencil } from 'lucide-react';

import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';

import { Command } from '@/types/command';
import { RemoveDialog } from './remove-dialog';
import { Label } from './ui/label';
import { Textarea } from './ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';
import { ParamViewer } from './param-viewer';
import { useEffect, useState } from 'react';
import { Parameter } from '@/types/parameter';
import { invoke } from '@tauri-apps/api/core';

interface CommandDisplayProps {
  command: Command | null;
}

export function CommandDisplay({ command }: CommandDisplayProps) {
  const [generatedCommand, setGeneratedCommand] = useState<string>("");
  const [parameters, setParameters] = useState<Parameter[]>([]);
  const [generatedValues, setGeneratedValues] = useState<string[]>([]);
  
  useEffect(() => {
    if (command) {
      invoke<[string, Parameter[], string[]]>('get_and_replace_parameters', {
        command: command.command,
      })
        .then((res) => {
          setGeneratedCommand(res[0]);
          setParameters(res[1]);
          setGeneratedValues(res[2]);
        })
        .catch((error) => console.error(error));
    }
  }, [command]);

  return (
    <div className="flex h-full flex-col">
      {command ? (
        <div className="flex flex-1 flex-col">
          <div className="flex items-center p-4">
            <div className="flex items-start gap-4 text-sm">
              <div className="grid gap-1">
                <div className="font-semibold">
                  {command.tag ? command.tag : 'Untagged'}
                </div>
              </div>
            </div>
            {command.last_used && (
              <div className="ml-auto text-xs text-muted-foreground">
                {format(new Date(command.last_used * 1000), 'PPpp')}
              </div>
            )}
            <Separator orientation="vertical" className="mx-2 h-6" />
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="ghost" size="icon" disabled={!command}>
                  <Pencil className="h-4 w-4" />
                  <span className="sr-only">Edit command</span>
                </Button>
              </TooltipTrigger>
              <TooltipContent>Edit command</TooltipContent>
            </Tooltip>
            <RemoveDialog command={command} />
          </div>
          <Separator />
          <div className="flex-1 whitespace-pre-wrap p-4 text-sm">
            <Label htmlFor="command">Command</Label>
            <Textarea
              id="command"
              className="p-4 resize-none"
              value={command.command}
              contentEditable={false}
            />
            <Label htmlFor="note">Note</Label>
            <Textarea
              id="note"
              className="p-4 resize-none"
              value={command.note}
              contentEditable={false}
            />
            <Label htmlFor="parameters">Parameters</Label>
            <ParamViewer parameters={parameters} generatedValues={generatedValues}/>
          </div>
          <Separator className="mt-auto" />
          <div className="p-4">
            <form>
              <div className="grid gap-4">
                <Textarea
                  className="p-4 resize-none"
                  value={generatedCommand}
                  disabled={true}
                />
                <div className="flex items-center">
                  {/* <Label
                    htmlFor="mute"
                    className="flex items-center gap-2 text-xs font-normal"
                  >
                    <Switch id="mute" aria-label="Mute thread" /> Mute this
                    thread
                  </Label> */}
                  <div className="ml-auto">
                    <Button onClick={(e) => e.preventDefault()} size="sm">
                      Copy
                    </Button>
                  </div>
                </div>
              </div>
            </form>
          </div>
        </div>
      ) : (
        <div className="p-8 text-center text-muted-foreground">
          No command selected
        </div>
      )}
    </div>
  );
}
