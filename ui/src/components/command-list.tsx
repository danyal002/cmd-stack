import { cn } from '@/lib/utils';
import { Badge } from '@/components/ui/badge';
import { invoke } from '@tauri-apps/api/core';
import { useCommand, useCommands } from '@/use-command';
import { Command } from '@/types/command';
import { Star } from 'lucide-react';

interface CommandListProps {
  items: Command[];
}

export function CommandList({ items }: CommandListProps) {
  const [command, setCommand] = useCommand();
  const [, refreshCommands] = useCommands();

  return (
    <div className="flex flex-col gap-2 p-4 pt-0">
      {items.map((item) => (
        <button
          key={item.id}
          className={cn(
            'group flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all hover:bg-accent',
            command.selected === item.id && 'bg-muted',
          )}
          onClick={() =>
            setCommand({
              ...command,
              selected: item.id,
            })
          }
        >
          <div className="flex w-full flex-col gap-1">
            <div className="flex items-center">
              <div className="truncate flex items-center gap-2">
                <div className="truncate font-semibold">{item.command}</div>
              </div>
            </div>
          </div>
          {item.note && (
            <div className="line-clamp-2 text-xs text-muted-foreground">
              {item.note.substring(0, 300)}
            </div>
          )}
          <div className="w-full flex items-center gap-2">
            <Badge key={item.tag} variant={'secondary'}>
              {item.tag ? item.tag : 'Untagged'}
            </Badge>
            <Star
              className={cn(
                'ml-auto h-3.5 hover:stroke-foreground stroke-muted-foreground invisible group-hover:visible',
                item.favourite &&
                  'visible fill-muted-foreground hover:fill-foreground',
              )}
              onClick={(e) => {
                e.stopPropagation();
                invoke('update_command', {
                  commandId: item.id,
                  command: {
                    ...item,
                    favourite: !item.favourite,
                  },
                })
                  .then((res) => {
                    console.log(res);
                    refreshCommands();
                  })
                  .catch((error) => {
                    console.log(error);
                  });
              }}
            />
          </div>
        </button>
      ))}
    </div>
  );
}
