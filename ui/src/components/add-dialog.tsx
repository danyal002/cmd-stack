import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from '@/components/ui/dialog';
import { AddForm } from './add-form';
import { Button } from '@/components/ui/button';
import { CirclePlus } from 'lucide-react';
interface AddDialogProps {}

export function AddDialog({}: AddDialogProps) {
  return (
    <Dialog>
      <div className="group flex flex-col gap-4 py-2 data-[collapsed=true]:py-2">
        <DialogTrigger className="grid gap-1 px-2 group-[[data-collapsed=true]]:justify-center group-[[data-collapsed=true]]:px-2">
          <Button variant={'default'} size={'sm'} className="justify-start">
            <CirclePlus className="mr-2 h-4 w-4" />
            Add Command
          </Button>
        </DialogTrigger>
      </div>
      <DialogContent>
        <DialogHeader>
          <AddForm />
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
