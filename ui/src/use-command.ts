import { atom, useAtom } from "jotai"
import { loadable } from 'jotai/utils'
import { invoke } from '@tauri-apps/api/core';

import { Command } from "./types/command"

type Config = {
  selected: Command["id"] | null
}

const configAtom = atom<Config>({
  selected: null,
})

export function useCommand() {
  return useAtom(configAtom)
}

const responseAsync = atom<Command[]>([]) 

const setAsyncCommandsAtom = atom(null, async (_get, set) => {
  const res = await invoke<Command[]>('list_commands')
  set(responseAsync, res)
});

const loadableCommandsAtom = loadable(responseAsync);

export function useCommands() {
  return useAtom(loadableCommandsAtom)
}

export function useRefresh() {
  return useAtom(setAsyncCommandsAtom)
}
