import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from '@/components/ui/dialog';
import { AddForm } from './add-form';
import { Button } from '@/components/ui/button';
import { CirclePlus } from 'lucide-react';
import { useState } from 'react';

interface AddDialogProps {}

export function AddDialog({}: AddDialogProps) {
  const [open, setOpen] = useState(false);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger>
        <Button variant="default" type="button" size="icon">
          <CirclePlus className="h-4 w-4" />
          <span className="sr-only">Add command</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <AddForm onSuccess={() => setOpen(false)} />
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
