import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function CommandList() {
  const [commands, setCommands] = useState<String[]>([]);

  useEffect(() => {
    invoke<String[]>('list_commands')
      .then((res) => {
        return setCommands(res);
      })
      .catch((error) => console.error(error));
  }, []);

  return (
    <>
      <h2>Available Commands</h2>
      <ul>
        {commands.length === 0 && <h3>No commands available</h3>}
        {commands.map((cmd, i) => (
          <h3 key={i}>{cmd}</h3>
        ))}
      </ul>
    </>
  );
}

export default CommandList;
