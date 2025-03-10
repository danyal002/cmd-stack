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
    <div className="py-2 px-4 space-y-4 rounded-md border mb-4">
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
    <div className="text-sm flex items-center">
      <Label className="font-normal w-[200px]">
        {parameter.type}{' '}
        {(parameter.type == ParameterType.String ||
          parameter.type == ParameterType.Int) &&
          `(Min: ${parameter.data.min.toString()}, Max: ${parameter.data.max.toString()})`}
      </Label>
      <Label className="pl-3 py-[11px] flex-1 font-robotomono font-bold overflow-auto">
        {generatedValue}
      </Label>
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
    <div className="text-sm flex items-center">
      <Label className="font-normal w-[200px]">{`Blank @{${
        blankIndex + 1
      }}`}</Label>
      <Input
        autoCapitalize="off"
        autoCorrect="off"
        placeholder="Fill in"
        value={blankParamValue}
        onChange={onChange}
        className="flex-1 font-robotomono placeholder:font-sans"
      />
    </div>
  );
}
