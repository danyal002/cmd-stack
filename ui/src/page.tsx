import { Mail } from "@/components/mail"
import { accounts } from "@/data"
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command } from "./types/command";

export default function MailPage() {
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
        <Mail
          accounts={accounts}
          commands={commands}
          defaultLayout={defaultLayout}
          defaultCollapsed={defaultCollapsed}
          navCollapsedSize={4}
        />
      </div>
    </>
  )
}
