import { Parameter, ParameterType } from '@/types/parameter';
import { Input } from './ui/input';
import { Label } from './ui/label';

interface ParamProps {
  parameter: Parameter;
  generatedValue: string;
}

export function Param({ parameter, generatedValue }: ParamProps) {
  return (
    <div className="text-sm">
      <Label>
        {parameter.type}{' '}
        {(parameter.type == ParameterType.String ||
          parameter.type == ParameterType.Int) &&
          `(Min: ${parameter.data.min.toString()}, Max: ${parameter.data.max.toString()})`}
      </Label>
      <Input disabled={true} value={generatedValue} />
    </div>
  );
}
