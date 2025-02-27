import { Input } from './ui/input';
import { Label } from './ui/label';

interface BlankParamProps {
  blankIndex: number;
  blankParamValue: string;
  setBlankParam: (index: number, value: string) => void;
}

export function BlankParam({
  blankIndex,
  blankParamValue,
  setBlankParam,
}: BlankParamProps) {
  function onChange(e: React.ChangeEvent<HTMLInputElement>): void {
    const input = e.target.value;
    setBlankParam(blankIndex, input);
  }

  return (
    <div className="text-sm">
      <Label>Blank</Label>
      <Input
        autoCapitalize="off"
        autoCorrect="off"
        disabled={false}
        value={blankParamValue}
        onChange={onChange}
      />
    </div>
  );
}
