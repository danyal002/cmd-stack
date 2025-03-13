'use client';

import { useSettings } from '@/use-command';
import { Settings } from 'lucide-react';
import { startTransition, Suspense } from 'react';
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
  const [_, refreshSettings] = useSettings();

  function onDialogClick() {
    startTransition(() => {
      refreshSettings();
    });
  }

  return (
    <Suspense>
      <Dialog>
        <DialogTrigger className="w-full" onClick={onDialogClick}>
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
                <Label>Application Theme</Label>
                <div className="ml-auto">
                  <ModeToggle />
                </div>
              </div>
            </DialogDescription>
          </DialogHeader>
        </DialogContent>
      </Dialog>
    </Suspense>
  );
}
