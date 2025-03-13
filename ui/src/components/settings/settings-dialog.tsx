'use client';

import { useSettings } from '@/use-command';
import { Settings } from 'lucide-react';
import { startTransition, Suspense } from 'react';
import { ThemeToggle } from './theme-toggle';
import { Nav } from '../nav';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '../ui/dialog';
import { Label } from '../ui/label';
import { TerminalToggle } from './terminal-toggle';

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
            <DialogDescription className="pt-4 space-y-4">
              <div className="flex items-center">
                <Label>Application Theme</Label>
                <div className="ml-auto">
                  <ThemeToggle />
                </div>
              </div>
              <div className="flex items-center">
                <Label>Default Terminal</Label>
                <div className="ml-auto">
                  <TerminalToggle />
                </div>
              </div>
            </DialogDescription>
          </DialogHeader>
        </DialogContent>
      </Dialog>
    </Suspense>
  );
}
