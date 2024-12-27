'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { toast } from '@/hooks/use-toast';
import { useRefresh } from '@/use-command';
import { Switch } from './ui/switch';

const FormSchema = z.object({
  command: z.string().min(1, {
    message: 'Command must be at least 1 character.',
  }),
  alias: z.string().min(1, {
    message: 'Alias must be at least 1 character.',
  }),
  tag: z.string(),
  note: z.string(),
  favourite: z.boolean(),
});

export function AddForm() {
  const [, refreshData] = useRefresh();

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      command: '',
      alias: '',
      tag: '',
      note: '',
      favourite: false,
    },
  });

  function onSubmit(data: z.infer<typeof FormSchema>) {
    invoke('add_command', { command: data })
      .then((res) => {
        console.log(res);
        toast({
          title: 'Command added ✅',
        });

        refreshData();
      })
      .catch((error) => {
        console.log(error);
        toast({
          title: 'Error ❌',
        });
      });
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="w-2/3 space-y-6">
        <FormField
          control={form.control}
          name="command"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Command</FormLabel>
              <FormControl>
                <Input placeholder="shadcn" {...field} />
              </FormControl>
              <FormDescription>This is your command.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="alias"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Alias</FormLabel>
              <FormControl>
                <Input placeholder="shadcn" {...field} />
              </FormControl>
              <FormDescription>
                This is your alias for the command.
              </FormDescription>
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
                <Input placeholder="shadcn" {...field} />
              </FormControl>
              <FormDescription>
                This is your tag for the command.
              </FormDescription>
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
                <Input placeholder="shadcn" {...field} />
              </FormControl>
              <FormDescription>
                This is your note for the command.
              </FormDescription>
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
                <FormLabel>Favourite</FormLabel>
                <FormDescription>
                  Add this command to favourites.
                </FormDescription>
              </div>
              <FormControl>
                <Switch
                  checked={field.value}
                  onCheckedChange={field.onChange}
                />
              </FormControl>
            </FormItem>
          )}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Form>
  );
}
