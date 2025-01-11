import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { toast } from '@/hooks/use-toast';
import { Command } from '@/types/command';
import { useCommand, useRefresh } from '@/use-command';
import { invoke } from '@tauri-apps/api/core';
import { Trash2 } from 'lucide-react';
import { useState } from 'react';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';

interface RemoveDialogProps {
  command: Command | null;
}

export function RemoveDialog({ command }: RemoveDialogProps) {
  const [, refreshData] = useRefresh();
  const [_, setCommand] = useCommand();
  const [open, setOpen] = useState(false);

  function onDelete() {
    invoke('delete_command', { command: { id: command?.id } })
      .then((res) => {
        console.log(res);
        toast({
          title: 'Command removed ✅',
        });

        refreshData();
        setCommand({ selected: null });
        setOpen(false);
      })
      .catch((error) => {
        console.log(error);
        toast({
          title: 'Error ❌',
        });
      });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger disabled={!command}>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="ghost" size="icon" disabled={!command}>
              <Trash2 className="h-4 w-4" />
              <span className="sr-only">Move to trash</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>Move to trash</TooltipContent>
        </Tooltip>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Deletion</DialogTitle>
          <DialogDescription>
            Are you sure you want to permanently delete this command? This
            action cannot be undone.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button onClick={onDelete}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
