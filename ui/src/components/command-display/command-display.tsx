import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { toast } from '@/hooks/use-toast';
import { Command } from '@/types/command';
import { Parameter, ParameterType } from '@/types/parameter';
import { useCommands } from '@/use-command';
import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import format from 'date-fns/format';
import { Pencil, RefreshCwIcon, Save } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { ParamViewer } from './param-viewer';
import { RemoveDialog } from '../remove-dialog';
import { Checkbox } from '../ui/checkbox';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '../ui/form';
import { Label } from '../ui/label';
import { Textarea } from '../ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { ScrollArea } from '../ui/scroll-area';
import { Badge } from '../ui/badge';
import { cn } from '@/lib/utils';
import { formatDistanceToNow, set } from 'date-fns';
import { UseCommandBox } from './use-command-box';

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
  const [editing, setEditing] = useState(false);
  const [, refreshCommands] = useCommands();

  const form = useForm<z.infer<typeof FormSchema>>({
    disabled: !editing,
    resolver: zodResolver(FormSchema),
    values: {
      command: command ? command.command : '',
      tag: command && command.tag ? command.tag : '',
      note: command && command.note ? command.note : '',
      favourite: command ? command.favourite : false,
    },
  });

  function onSubmit(data: z.infer<typeof FormSchema>) {
    invoke('update_command', { commandId: command?.id, command: data })
      .then((res) => {
        console.log(res);
        toast({
          title: 'Command updated ✅ ',
        });

        refreshCommands();
        setEditing(false);
      })
      .catch((error) => {
        console.log(error);
        toast({
          title: `${error} ❌`,
        });
      });
  }

  function setBlankParam(index: number, value: string) {
    // https://react.dev/learn/updating-arrays-in-state#replacing-items-in-an-array
    let newBlankParamValues = blankParamValues.map((v, i) => {
      if (i == index) {
        return value;
      } else {
        return v;
      }
    });

    setBlankParamValues(newBlankParamValues);
  }

  const [parameterRefreshNumber, setParameterRefreshNumber] =
    useState<number>(0);

  const [generatedCommand, setGeneratedCommand] = useState<string>('');
  const [parameters, setParameters] = useState<Parameter[]>([]);
  const [generatedValues, setGeneratedValues] = useState<string[]>([]);
  const [blankParamValues, setBlankParamValues] = useState<string[]>([]);
  const [indexedCommand, setIndexedCommand] = useState('');

  useEffect(() => {
    if (command) {
      invoke<string>('index_blank_parameters', { command: command.command })
        .then((res) => {
          setIndexedCommand(res);
        })
        .catch((error) => console.error(error));
    }
  }, [command]);

  // This effect handles parsing the parameters
  useEffect(() => {
    if (command) {
      invoke<[string[], Parameter[]]>('parse_parameters', {
        command: command.command,
      })
        .then((res) => {
          const parameters = res[1];
          const numberOfBlankParameters = parameters.filter(
            (p) => p.type == ParameterType.Blank,
          ).length;

          setBlankParamValues(Array(numberOfBlankParameters).fill(''));
          setParameters(res[1]);
        })
        .catch((error) => console.error(error));
    }
  }, [command]);

  // This effect handles generating parameters
  useEffect(() => {
    if (command) {
      const blanks = parameters.filter(
        (p) => p.type == ParameterType.Blank,
      ).length;
      const blankParamValues: string[] = Array(blanks).fill('');

      invoke<[string, string[]]>('generate_parameters', {
        command: command.command,
        blankParamValues: blankParamValues,
      })
        .then((res) => {
          const generatedValues = res[1];
          setGeneratedValues(generatedValues);
        })
        .catch((error) => console.error(error));
    }
  }, [parameters, parameterRefreshNumber]);

  // This effect handles updating the generated command based on user editing
  useEffect(() => {
    if (command && generatedValues.length == parameters.length) {
      // Replace blank parameters
      let blankIndex = 0;
      const paramValues: string[] = parameters.map((p, index) =>
        p.type === ParameterType.Blank
          ? blankParamValues[blankIndex++]
          : generatedValues[index],
      );

      invoke<string>('replace_parameters', {
        command: command.command,
        paramValues: paramValues,
      })
        .then((res) => {
          let generatedCommand = res;
          setGeneratedCommand(generatedCommand);
        })
        .catch((error) => console.error(error));
    }
  }, [generatedValues, blankParamValues]);

  // This effect handles getting out of the editing state if we switch commands
  useEffect(() => {
    setEditing(false);
  }, [command]);

  function onParameterRefresh() {
    setParameterRefreshNumber(parameterRefreshNumber + 1);
  }

  function onEditing() {
    if (command) {
      setEditing(true);
    }
  }

  const tagParts = command?.tag ? command.tag.split('/') : [];

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-2">
        <div className="flex h-screen flex-col">
          {command ? (
            <>
              <div className="flex items-center p-2">
                <div className="flex items-center gap-2">
                  <div className="pl-2 font-semibold">
                    {command.tag &&
                      tagParts.map((tag, index) => (
                        <>
                          <Badge
                            key={index}
                            variant={
                              index == tagParts.length - 1
                                ? 'outline'
                                : 'secondary'
                            }
                            className={cn(
                              index !== tagParts.length - 1 &&
                                'text-secondary-foreground/40',
                            )}
                          >
                            {tag}
                          </Badge>
                          {index !== tagParts.length - 1 && (
                            <span className="text-xs font-semibold px-1.5">
                              /
                            </span>
                          )}
                        </>
                      ))}
                  </div>
                </div>
                {command.last_used && (
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <div className="ml-auto text-xs text-muted-foreground">
                        Last used&nbsp;
                        {formatDistanceToNow(
                          new Date(command.last_used * 1000),
                          {
                            addSuffix: true,
                          },
                        )}
                      </div>
                    </TooltipTrigger>
                    <TooltipContent>
                      {format(new Date(command.last_used * 1000), 'PPp')}
                    </TooltipContent>
                  </Tooltip>
                )}
                <Separator orientation="vertical" className="mx-2 h-6" />
                {!editing && (
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        variant="ghost"
                        type="button"
                        size="icon"
                        onClick={onEditing}
                      >
                        <Pencil className="h-4 w-4" />
                        <span className="sr-only">Edit command</span>
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>Edit command</TooltipContent>
                  </Tooltip>
                )}
                {editing && (
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button variant="ghost" type="submit" size="icon">
                        <Save className="h-4 w-4" />
                        <span className="sr-only">Save command</span>
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>Save command</TooltipContent>
                  </Tooltip>
                )}
                <RemoveDialog command={command} />
              </div>
              <Separator />
              <div className="flex flex-1 overflow-hidden flex-col">
                <div className="p-4 pb-0">
                  <UseCommandBox
                    command={generatedCommand}
                    commandId={command.id}
                    onChangeCommand={(e) => setGeneratedCommand(e.target.value)}
                    disabled={editing}
                  />
                  <FormField
                    control={form.control}
                    name="command"
                    render={({ field }) => {
                      const {
                        ref: fieldRef,
                        value: fieldValue,
                        ...rest
                      } = field;
                      return (
                        <FormItem>
                          <FormControl>
                            <Textarea
                              className="min-h-0 py-[7px] font-spacemono border-none resize-none shadow-none"
                              ref={(textarea) => {
                                fieldRef(textarea);
                                if (textarea) {
                                  textarea.style.height = '0px';
                                  textarea.style.height =
                                    textarea.scrollHeight + 2 + 'px';
                                }
                              }}
                              value={editing ? fieldValue : indexedCommand}
                              {...rest}
                            />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      );
                    }}
                  />
                </div>
                <ScrollArea className="flex-1">
                  <div className="p-4">
                    {editing ? (
                      <FormField
                        control={form.control}
                        name="note"
                        render={({ field }) => {
                          const { ref: fieldRef, ...rest } = field;
                          return (
                            <FormItem className="mb-4">
                              <FormControl>
                                <Textarea
                                  className="min-h-0 resize-none"
                                  placeholder="Add a note"
                                  ref={(textarea) => {
                                    fieldRef(textarea);
                                    if (textarea) {
                                      textarea.style.height = '0px';
                                      textarea.style.height =
                                        textarea.scrollHeight + 2 + 'px';
                                    }
                                  }}
                                  {...rest}
                                />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          );
                        }}
                      />
                    ) : command.note ? (
                      <div className="whitespace-pre-wrap text-sm mb-2">
                        {command.note}
                      </div>
                    ) : (
                      <div
                        className="text-sm mb-2 underline cursor-pointer w-fit"
                        onClick={() => setEditing(true)}
                      >
                        + Add a note
                      </div>
                    )}
                    {parameters.length > 0 && !editing && (
                      <>
                        <div className="flex items-center">
                          <Label htmlFor="parameters">Parameters</Label>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant="ghost"
                                size="icon"
                                type="button"
                                onClick={onParameterRefresh}
                              >
                                <RefreshCwIcon size={12} />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              Regenerate Parameters
                            </TooltipContent>
                          </Tooltip>
                        </div>
                        <ParamViewer
                          parameters={parameters}
                          generatedValues={generatedValues}
                          blankParamValues={blankParamValues}
                          setBlankParam={setBlankParam}
                        />
                      </>
                    )}
                    {/* Intentionally commented out for now
                    <FormField
                      control={form.control}
                      name="tag"
                      render={({ field }) => (
                        <FormItem className="mb-4">
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
                        <FormItem className="mb-4">
                          <div className="flex items-center">
                            <FormLabel className="mr-2">Favourite</FormLabel>
                            <FormControl>
                              <Checkbox
                                // Not sure why I need the disabled flag for a Checkbox but not Input
                                disabled={!editing}
                                checked={field.value}
                                onCheckedChange={field.onChange}
                              />
                            </FormControl>
                            <FormMessage />
                          </div>
                        </FormItem>
                      )}
                    /> */}
                  </div>
                </ScrollArea>
              </div>
            </>
          ) : (
            <div className="p-8 text-center text-muted-foreground">
              No command selected
            </div>
          )}
        </div>
      </form>
    </Form>
  );
}
