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
import { Checkbox } from './ui/checkbox';
import { Separator } from './ui/separator';
import { Plus } from 'lucide-react';

const FormSchema = z.object({
  command: z.string().min(1, {
    message: 'Command must be at least 1 character.',
  }),
  tag: z.string(),
  note: z.string(),
  favourite: z.boolean(),
});

interface AddFormProps {
  onSuccess: () => void;
}

export function AddForm({ onSuccess }: AddFormProps) {
  const [, refreshData] = useRefresh();

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      command: '',
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
          title: 'Command added ✅ ',
        });

        refreshData();

        onSuccess();
      })
      .catch((error) => {
        console.log(error);
        toast({
          title: `${error} ❌`,
        });
      });
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        <FormField
          control={form.control}
          name="command"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Command</FormLabel>
              <FormControl>
                <Input
                  placeholder="INSERT INTO users VALUES (@{string[5,10]}, @{int});"
                  {...field}
                />
              </FormControl>
              <FormDescription>This is your command.</FormDescription>
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
                <Input placeholder="SQL/INSERT" {...field} />
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
                <Input
                  placeholder="SQL for inserting a random user"
                  {...field}
                />
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
            <FormItem>
              <div className="flex items-center">
                <FormLabel className="mr-2">Favourite</FormLabel>
                <FormControl>
                  <Checkbox
                    checked={field.value}
                    onCheckedChange={field.onChange}
                  />
                </FormControl>
                <FormMessage />
              </div>
              <FormDescription>
                Add this command to favourites.
              </FormDescription>
            </FormItem>
          )}
        />
        <Separator />
        <div className="flex justify-center">
          <Button className="w-1/2" type="submit">
            Add Command
            <Plus className="h-4 w-4" />
          </Button>
        </div>
      </form>
    </Form>
  );
}
