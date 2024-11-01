import { atom, useAtom } from "jotai"

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
