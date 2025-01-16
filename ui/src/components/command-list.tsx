import { ComponentProps } from 'react';
import formatDistanceToNow from 'date-fns/formatDistanceToNow';

import { cn } from '@/lib/utils';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useCommand } from '@/use-command';
import { Command } from '@/types/command';

interface CommandListProps {
  items: Command[];
  selectedTag: string | undefined;
}

export function CommandList({ items, selectedTag }: CommandListProps) {
  const [command, setCommand] = useCommand();

  return (
    <ScrollArea className="h-[calc(100vh-121px)]">
      <div className="flex flex-col gap-2 p-4 pt-0">
        {selectedTag && (
          <div className="flex items-center gap-2">
            <Badge variant="secondary">{selectedTag}</Badge>
          </div>
        )}
        {items.map((item) => (
          <button
            key={item.id}
            className={cn(
              'flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all hover:bg-accent',
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
                <div className="flex items-center gap-2">
                  <div className="font-semibold">{item.alias}</div>
                </div>
                <div
                  className={cn(
                    'ml-auto text-xs',
                    command.selected === item.id
                      ? 'text-foreground'
                      : 'text-muted-foreground',
                  )}
                >
                  {formatDistanceToNow(new Date(item.last_used * 1000), {
                    addSuffix: true,
                  })}
                </div>
              </div>
              <div className="text-xs font-medium">{item.command}</div>
            </div>
            {item.note && (
              <div className="line-clamp-2 text-xs text-muted-foreground">
                {item.note.substring(0, 300)}
              </div>
            )}
            {item.tag ? (
              <div className="flex items-center gap-2">
                <Badge
                  key={item.tag}
                  variant={getBadgeVariantFromLabel(item.tag)}
                >
                  {item.tag}
                </Badge>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <Badge variant={'secondary'}>{'Untagged'}</Badge>
              </div>
            )}
          </button>
        ))}
      </div>
    </ScrollArea>
  );
}

function getBadgeVariantFromLabel(
  label: string,
): ComponentProps<typeof Badge>['variant'] {
  if (['work'].includes(label.toLowerCase())) {
    return 'default';
  }

  if (['personal'].includes(label.toLowerCase())) {
    return 'outline';
  }

  return 'secondary';
}
