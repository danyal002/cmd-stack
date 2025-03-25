import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { toast } from '@/hooks/use-toast';
import { cn } from '@/lib/utils';
import { Command } from '@/types/command';
import { Parameter, ParameterType } from '@/types/parameter';
import { useCommands } from '@/use-command';
import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { formatDistanceToNow } from 'date-fns';
import format from 'date-fns/format';
import { Pencil, RefreshCwIcon, Save, X } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { RemoveDialog } from '../remove-dialog';
import { Badge } from '../ui/badge';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '../ui/form';
import { Label } from '../ui/label';
import { ScrollArea } from '../ui/scroll-area';
import { Textarea } from '../ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { ParamViewer } from './param-viewer';
import { UseCommandBox } from './use-command-box';
import { Input } from '../ui/input';

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
  const [editing, setEditing] = useState({
    note: false,
    tag: false,
    command: false,
  });
  const [, refreshCommands] = useCommands();

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    values: {
      command: command ? command.command : '',
      tag: command && command.tag ? command.tag : '',
      note: command && command.note ? command.note : '',
      favourite: command ? command.favourite : false,
    },
  });

  const tagRef = useRef<HTMLInputElement | null>(null);
  const noteRef = useRef<HTMLTextAreaElement | null>(null);
  const commandRef = useRef<HTMLTextAreaElement | null>(null);

  function onSubmit(data: z.infer<typeof FormSchema>) {
    invoke('update_command', { commandId: command?.id, command: data })
      .then((res) => {
        console.log(res);
        toast({
          title: 'Command updated ✅ ',
        });

        refreshCommands();
        setEditing({
          note: false,
          tag: false,
          command: false,
        });
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
  const [indexedCommand, setIndexedCommand] = useState<string>('');

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
    setEditing({
      note: false,
      tag: false,
      command: false,
    });
  }, [command]);

  // These effects handles focusing the correct input when we edit anything
  useEffect(() => {
    if (editing.command) commandRef.current!.focus();
  }, [editing.command]);

  useEffect(() => {
    if (editing.tag && !editing.command) tagRef.current!.focus();
  }, [editing.tag]);

  useEffect(() => {
    if (editing.note && !editing.command) noteRef.current!.focus();
  }, [editing.note]);

  function onParameterRefresh() {
    setParameterRefreshNumber(parameterRefreshNumber + 1);
  }

  function onEditing() {
    if (command) {
      setEditing({
        note: true,
        tag: true,
        command: true,
      });
    }
  }

  function onCancelEdit() {
    setEditing({
      note: false,
      tag: false,
      command: false,
    });
    form.reset();
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
                  <div className="pl-2">
                    {editing.tag ? (
                      <FormField
                        control={form.control}
                        name="tag"
                        render={({ field }) => {
                          const { ref: fieldRef, ...rest } = field;
                          return (
                            <FormItem>
                              <FormControl>
                                <Input
                                  placeholder="Add a tag"
                                  {...rest}
                                  ref={(input) => {
                                    fieldRef(input);
                                    tagRef.current = input;
                                  }}
                                  autoCapitalize="off"
                                  autoCorrect="off"
                                />
                              </FormControl>
                              <FormMessage />
                            </FormItem>
                          );
                        }}
                      />
                    ) : command.tag ? (
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
                      ))
                    ) : (
                      <div
                        className="text-sm underline cursor-pointer w-fit"
                        onClick={() =>
                          setEditing({
                            ...editing,
                            tag: true,
                          })
                        }
                      >
                        + Add a tag
                      </div>
                    )}
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
                {!(editing.command || editing.tag || editing.note) ? (
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
                ) : (
                  <>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button variant="ghost" type="submit" size="icon">
                          <Save className="h-4 w-4" />
                          <span className="sr-only">Save command</span>
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent>Save command</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button
                          variant="ghost"
                          type="button"
                          size="icon"
                          onClick={onCancelEdit}
                        >
                          <X className="h-4 w-4" />
                          <span className="sr-only">Cancel editing</span>
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent>Cancel editing</TooltipContent>
                    </Tooltip>
                  </>
                )}
                <RemoveDialog command={command} />
              </div>
              <Separator />
              <div className="flex flex-1 overflow-hidden flex-col">
                <div className="p-4 pb-0">
                  {!editing.command && (
                    <UseCommandBox
                      command={generatedCommand}
                      commandId={command.id}
                      onChangeCommand={(e) =>
                        setGeneratedCommand(e.target.value)
                      }
                    />
                  )}
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
                              className={cn(
                                !editing.command && 'border-none shadow-none',
                                'min-h-0 py-[7px] font-robotomono resize-none disabled:cursor-auto',
                              )}
                              ref={(textarea) => {
                                fieldRef(textarea);
                                commandRef.current = textarea;
                                if (textarea) {
                                  textarea.style.height = '0px';
                                  textarea.style.height =
                                    textarea.scrollHeight + 2 + 'px';
                                }
                              }}
                              value={
                                editing.command ? fieldValue : indexedCommand
                              }
                              disabled={!editing.command}
                              autoCapitalize="off"
                              autoCorrect="off"
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
                    {parameters.length > 0 && !editing.command && (
                      <>
                        <div className="flex items-center h-9">
                          <Label htmlFor="parameters">Parameters</Label>
                          {/* Only allow regenerating parameters if there are non-blank parameters */}
                          {parameters.some(
                            (p) => p.type !== ParameterType.Blank,
                          ) && (
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
                          )}
                        </div>
                        <ParamViewer
                          parameters={parameters}
                          generatedValues={generatedValues}
                          blankParamValues={blankParamValues}
                          setBlankParam={setBlankParam}
                        />
                      </>
                    )}
                    <FormField
                      control={form.control}
                      name="note"
                      render={({ field }) => {
                        const { ref: fieldRef, ...rest } = field;
                        return (
                          <FormItem className="mb-4">
                            <FormLabel>Note</FormLabel>
                            {command.note || editing.note ? (
                              <FormControl>
                                <Textarea
                                  className={cn(
                                    !editing.note && 'border-none shadow-none',
                                    'min-h-0 resize-none disabled:opacity-100 disabled:cursor-default',
                                  )}
                                  placeholder="Add a note"
                                  ref={(textarea) => {
                                    fieldRef(textarea);
                                    noteRef.current = textarea;
                                    if (textarea) {
                                      textarea.style.height = '0px';
                                      textarea.style.height =
                                        textarea.scrollHeight + 2 + 'px';
                                    }
                                  }}
                                  disabled={!editing.note}
                                  {...rest}
                                />
                              </FormControl>
                            ) : (
                              <div
                                className="text-sm underline cursor-pointer w-fit"
                                onClick={() =>
                                  setEditing({
                                    ...editing,
                                    note: true,
                                  })
                                }
                              >
                                + Add a note
                              </div>
                            )}
                            <FormMessage />
                          </FormItem>
                        );
                      }}
                    />
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
