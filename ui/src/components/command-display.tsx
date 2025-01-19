import format from 'date-fns/format';
import { MoreVertical } from 'lucide-react';

import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Separator } from '@/components/ui/separator';

import { Command } from '@/types/command';
import { RemoveDialog } from './remove-dialog';

interface CommandDisplayProps {
  command: Command | null;
}

export function CommandDisplay({ command }: CommandDisplayProps) {
  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center p-2">
        <div className="flex items-center gap-2">
          <RemoveDialog command={command} />
        </div>
        <div className="ml-auto flex items-center gap-2"></div>
        <Separator orientation="vertical" className="mx-2 h-6" />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon" disabled={!command}>
              <MoreVertical className="h-4 w-4" />
              <span className="sr-only">More</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem>Option 1</DropdownMenuItem>
            <DropdownMenuItem>Option 2</DropdownMenuItem>
            <DropdownMenuItem>Option 3</DropdownMenuItem>
            <DropdownMenuItem>Option 4</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <Separator />
      {command ? (
        <div className="flex flex-1 flex-col">
          <div className="flex items-start p-4">
            <div className="flex items-start gap-4 text-sm">
              <Avatar>
                <AvatarImage alt={command.id} />
                <AvatarFallback>{command.id}</AvatarFallback>
              </Avatar>
              <div className="grid gap-1">
                <div className="font-semibold">{command.command}</div>
              </div>
            </div>
            {command.last_used && (
              <div className="ml-auto text-xs text-muted-foreground">
                {format(new Date(command.last_used * 1000), 'PPpp')}
              </div>
            )}
          </div>
          <Separator />
          <div className="flex-1 whitespace-pre-wrap p-4 text-sm">
            {command.note}
          </div>
          <Separator className="mt-auto" />
          {/* <div className="p-4">
            <form>
              <div className="grid gap-4">
                <Textarea
                  className="p-4"
                  placeholder={`Reply ${command.name}...`}
                />
                <div className="flex items-center">
                  <Label
                    htmlFor="mute"
                    className="flex items-center gap-2 text-xs font-normal"
                  >
                    <Switch id="mute" aria-label="Mute thread" /> Mute this
                    thread
                  </Label>
                  <Button
                    onClick={(e) => e.preventDefault()}
                    size="sm"
                    className="ml-auto"
                  >
                    Send
                  </Button>
                </div>
              </div>
            </form>
          </div> */}
        </div>
      ) : (
        <div className="p-8 text-center text-muted-foreground">
          No command selected
        </div>
      )}
    </div>
  );
}
