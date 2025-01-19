import format from 'date-fns/format';
import { Pencil, RefreshCwIcon } from 'lucide-react';

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
import { toast } from '@/hooks/use-toast';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useForm } from 'react-hook-form';
import {
  Form,
  FormControl, FormField,
  FormItem,
  FormLabel,
  FormMessage
} from './ui/form';
import { Input } from './ui/input';
import { Switch } from './ui/switch';

interface CommandDisplayProps {
  command: Command | null;
}

const FormSchema = z.object({
  command: z.string().min(1, {
    message: 'Command must be at least 1 character.',
  }),
  tag: z.string(),
  note: z.string(),
  favourite: z.boolean(),
});

export function CommandDisplay({ command }: CommandDisplayProps) {
  const form = useForm<z.infer<typeof FormSchema>>({
    disabled: true,
    resolver: zodResolver(FormSchema),
    values: {
      command: command ? command.command : '',
      tag: command && command.tag ? command.tag : '',
      note: command && command.note ? command.note : '',
      favourite: command ? command.favourite : false,
    },
  });

  function onSubmit(data: z.infer<typeof FormSchema>) {
    console.log('Form submitted!');
  }

  const [parameterRefreshNumber, setParameterRefreshNumber] =
    useState<number>(0);

  const [generatedCommand, setGeneratedCommand] = useState<string>('');
  const [parameters, setParameters] = useState<Parameter[]>([]);
  const [generatedValues, setGeneratedValues] = useState<string[]>([]);

  // This effect handles parsing the parameters
  useEffect(() => {
    if (command) {
      invoke<[string[], Parameter[]]>('parse_parameters', {
        command: command.command,
      })
        .then((res) => {
          setParameters(res[1]);
        })
        .catch((error) => console.error(error));
    }
  }, [command]);

  // This effect handles generating parameters
  useEffect(() => {
    if (command) {
      invoke<[string, string[]]>('replace_parameters', {
        command: command.command,
      })
        .then((res) => {
          setGeneratedCommand(res[0]);
          setGeneratedValues(res[1]);
        })
        .catch((error) => console.error(error));
    }
  }, [command, parameterRefreshNumber]);

  function onParameterRefresh() {
    setParameterRefreshNumber(parameterRefreshNumber + 1);
  }

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
                <Button variant="ghost" size="icon" disabled={true}>
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
            <Form {...form}>
              <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="space-y-2"
              >
                <FormField
                  control={form.control}
                  name="command"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Command</FormLabel>
                      <FormControl>
                        <Textarea
                          className="resize-none"
                          placeholder=""
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="note"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Note</FormLabel>
                      <FormControl>
                        <Textarea
                          className="resize-none"
                          placeholder=""
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="tag"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Tag</FormLabel>
                      <FormControl>
                        <Input placeholder="" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="favourite"
                  render={({ field }) => (
                    <FormItem className="flex flex-row items-center justify-between rounded-lg border p-3 shadow-sm">
                      <div className="space-y-0.5">
                        <FormLabel>Favourite ❤️</FormLabel>
                      </div>
                      <FormControl>
                        <Switch
                          // Not sure why I need the disabled flag for a Switch but not Input
                          disabled={true}
                          checked={field.value}
                          onCheckedChange={field.onChange}
                        />
                      </FormControl>
                    </FormItem>
                  )}
                />
              </form>
            </Form>
            <div className="flex items-center">
              <Label htmlFor="parameters" className="mr-2">
                Parameters
              </Label>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="icon"
                    disabled={parameters.length == 0}
                    onClick={onParameterRefresh}
                  >
                    <RefreshCwIcon size={12} />
                  </Button>
                </TooltipTrigger>
              </Tooltip>
            </div>
            <ParamViewer
              parameters={parameters}
              generatedValues={generatedValues}
            />
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
                  <div className="ml-auto">
                    <Button
                      onClick={(e) => {
                        e.preventDefault();
                        navigator.clipboard.writeText(generatedCommand);
                        toast({
                          title: 'Copied ✅',
                        });
                      }}
                      size="sm"
                    >
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
