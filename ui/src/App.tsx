import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './index.css'
import MailPage from './page';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke('greet', { name }));
  }

  return (
    <main className="container">
      <MailPage />
    </main>
  );
}

export default App;
