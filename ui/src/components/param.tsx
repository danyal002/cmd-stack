import { Parameter, ParameterType } from '@/types/parameter';
import { Input } from './ui/input';
import { Label } from './ui/label';

interface ParamProps {
  parameter: Parameter;
  index: Number;
  generatedValue: string;
}

export function Param({ parameter, index, generatedValue }: ParamProps) {
  return (
    <div key={index.toString()} className="text-sm">
      <Label>
        {parameter.type}{' '}
        {(parameter.type == ParameterType.String ||
          parameter.type == ParameterType.Int) &&
          `Min: ${parameter.data.min.toString()}, Max: ${parameter.data.max.toString()})`}
      </Label>
      <Input value={generatedValue} />
    </div>
  );
}
