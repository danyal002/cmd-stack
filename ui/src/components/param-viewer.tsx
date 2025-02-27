import { BlankParam } from './blank-param';
import { Param } from './param';
import { ScrollArea } from './ui/scroll-area';
import { Parameter, ParameterType } from '@/types/parameter';

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
    <ScrollArea id="parameters" className="h-40 rounded-md border">
      <div className="p-4">
        {parameters.map((parameter, index) => {
          if (parameter.type == ParameterType.Blank) {
            let blankIndex = parameters.filter((p, i) => p.type == ParameterType.Blank && i < index).length;

            return (
              <BlankParam
                setBlankParam={setBlankParam}
                blankIndex={blankIndex}
                blankParamValue={blankParamValues[blankIndex]}
              />
            );
          } else {
            return (
              <Param
                parameter={parameter}
                generatedValue={
                  index < generatedValues.length ? generatedValues[index] : ''
                }
              />
            );
          }
        })}
      </div>
    </ScrollArea>
  );
}
