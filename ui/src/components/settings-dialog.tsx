'use client';

import { Settings } from 'lucide-react';
import { ModeToggle } from './mode-toggle';
import { Nav } from './nav';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from './ui/dialog';
import { Label } from './ui/label';

interface SettingsDialogProps {
  isCollapsed: boolean;
}

export function SettingsDialog({ isCollapsed }: SettingsDialogProps) {
  return (
    <Dialog>
      <DialogTrigger className="w-full">
        <Nav
          isCollapsed={isCollapsed}
          links={[
            {
              title: 'Settings',
              icon: Settings,
              variant: 'ghost',
              onClick() {},
            },
          ]}
        />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
          <DialogDescription>
            <div className="flex items-center">
              <Label>Theme</Label>
              <div className="m-2">
                <ModeToggle />
              </div>
            </div>
          </DialogDescription>
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}
