import { Parameter, ParameterType } from '@/types/parameter';
import { Label } from '../ui/label';
import { Input } from '../ui/input';

interface ParamViewerProps {
  parameters: Parameter[];
  generatedValues: string[];
  blankParamValues: string[];
  setBlankParam: (index: number, value: string) => void;
}

export function ParamViewer({
  parameters,
  generatedValues,
  blankParamValues,
  setBlankParam,
}: ParamViewerProps) {
  return (
    <div className="p-4 space-y-4 rounded-md border">
      {parameters.map((parameter, index) => {
        if (parameter.type == ParameterType.Blank) {
          let blankIndex = parameters
            .slice(0, index)
            .filter((p) => p.type == ParameterType.Blank).length;

          return (
            <BlankParam
              key={index}
              setBlankParam={setBlankParam}
              blankIndex={blankIndex}
              blankParamValue={blankParamValues[blankIndex]}
            />
          );
        } else {
          return (
            <Param
              key={index}
              parameter={parameter}
              generatedValue={generatedValues[index] ?? ''}
            />
          );
        }
      })}
    </div>
  );
}

interface ParamProps {
  parameter: Parameter;
  generatedValue: string;
}

function Param({ parameter, generatedValue }: ParamProps) {
  return (
    <div className="text-sm flex justify-between items-center">
      <Label className="font-normal">
        {parameter.type}{' '}
        {(parameter.type == ParameterType.String ||
          parameter.type == ParameterType.Int) &&
          `(Min: ${parameter.data.min.toString()}, Max: ${parameter.data.max.toString()})`}
      </Label>
      <Input
        disabled={true}
        value={generatedValue}
        className="w-auto disabled:opacity-100 font-spacemono border-none shadow-none font-bold"
      />
    </div>
  );
}

interface BlankParamProps {
  blankIndex: number;
  blankParamValue: string;
  setBlankParam: (index: number, value: string) => void;
}

function BlankParam({
  blankIndex,
  blankParamValue,
  setBlankParam,
}: BlankParamProps) {
  function onChange(e: React.ChangeEvent<HTMLInputElement>): void {
    const input = e.target.value;
    setBlankParam(blankIndex, input);
  }

  return (
    <div className="text-sm flex justify-between items-center">
      <Label className="font-normal">{`Blank @{${blankIndex + 1}}`}</Label>
      <Input
        autoCapitalize="off"
        autoCorrect="off"
        placeholder="Fill in"
        value={blankParamValue}
        onChange={onChange}
        className="w-auto font-spacemono placeholder:font-sans"
      />
    </div>
  );
}
