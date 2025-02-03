import { Param } from './param';
import { ScrollArea } from './ui/scroll-area';
import { Parameter } from '@/types/parameter';

interface ParamViewerProps {
  parameters: Parameter[];
  generatedValues: string[];
}

export function ParamViewer({ parameters, generatedValues }: ParamViewerProps) {
  return (
    <ScrollArea id="parameters" className="h-40 rounded-md border">
      <div className="p-4">
        {parameters.map((parameter, index) => (
          <Param
            key={index}
            parameter={parameter}
            generatedValue={
              index < generatedValues.length ? generatedValues[index] : ''
            }
            index={index}
          />
        ))}
      </div>
    </ScrollArea>
  );
}
