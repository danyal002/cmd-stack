export type Command = {
  id: string;
  last_used: number;
  command: string;
  tag?: string;
  note?: string;
  favourite: boolean;
};
