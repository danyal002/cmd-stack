export type Command = {
    id: string
    last_used: number,
    alias: string,
    command: string,
    tag?: string,
    note?: string,
    favourite: boolean,
}
