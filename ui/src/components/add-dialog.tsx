import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from '@/components/ui/dialog';
import { AddForm } from './add-form';
import { Button } from "@/components/ui/button";

interface AddDialogProps {}

export function AddDialog({}: AddDialogProps) {
  return (
    <Dialog>
      <DialogTrigger>
        <Button>Add Command</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <AddForm />
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
