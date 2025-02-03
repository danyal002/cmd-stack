import { atom, useAtom } from 'jotai';
import { atomWithRefresh } from 'jotai/utils';
import { invoke } from '@tauri-apps/api/core';

import { Command } from './types/command';

type Config = {
  selected: Command['id'] | null;
};

const configAtom = atom<Config>({
  selected: null,
});

export function useCommand() {
  return useAtom(configAtom);
}

const searchAtom = atom('');

export function useSearch() {
  return useAtom(searchAtom);
}

const commandsAtom = atomWithRefresh((get) => {
  const search = get(searchAtom);
  return invoke<Command[]>('search_commands', { search: search }).then(
    (r) => r,
  );
});

export function useCommands() {
  return useAtom(commandsAtom);
}
