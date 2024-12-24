import { MainCommandPage } from "@/components/command"
import { accounts } from "@/data"
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command } from "./types/command";
import { Toaster } from "./components/ui/toaster";

export default function CommandPage() {
  const [commands, setCommands] = useState<Command[]>([]);

  useEffect(() => {
    invoke<Command[]>('list_commands')
      .then((res) => {
        return setCommands(res);
      })
      .catch((error) => console.error(error));
  }, []);

  const defaultLayout = undefined
  const defaultCollapsed = undefined

  return (
    <>
      <div className="hidden flex-col md:flex">
        <MainCommandPage
          accounts={accounts}
          commands={commands}
          defaultLayout={defaultLayout}
          defaultCollapsed={defaultCollapsed}
          navCollapsedSize={4}
        />
        <Toaster />
      </div>
    </>
  )
}
