import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from '@/components/ui/dialog';
import { AddForm } from './add-form';
import { Button } from '@/components/ui/button';
import { Plus } from 'lucide-react';
import { useState } from 'react';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';

interface AddDialogProps {}

export function AddDialog({}: AddDialogProps) {
  const [open, setOpen] = useState(false);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="ghost" type="button" size="icon">
              <Plus className="h-4 w-4" />
              <span className="sr-only">Add command</span>
            </Button>
          </TooltipTrigger>
          <TooltipContent>Add command</TooltipContent>
        </Tooltip>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <AddForm onSuccess={() => setOpen(false)} />
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
