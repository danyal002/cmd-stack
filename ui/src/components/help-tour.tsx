import { X } from 'lucide-react';
import Joyride, {
  CallBackProps,
  STATUS,
  TooltipRenderProps,
} from 'react-joyride';
import { Button } from './ui/button';
import { Command } from '@/types/command';

function TourTooltip({
  step,
  backProps,
  skipProps,
  index,
  primaryProps,
}: TooltipRenderProps) {
  return (
    <div className="relative bg-background shadow-lg rounded-lg p-4 pt-8">
      <button
        className="absolute top-2 right-2 p-1 text-muted-foreground hover:text-foreground"
        {...skipProps}
      >
        <X size={16} />
      </button>

      <p className="text-sm text-foreground">{step.content}</p>
      <div className="flex justify-end gap-2 mt-4">
        {index > 0 && (
          <Button variant="outline" size="sm" {...backProps}>
            {backProps.title}
          </Button>
        )}
        <Button size="sm" {...primaryProps}>
          {primaryProps.title}
        </Button>
      </div>
    </div>
  );
}

interface HelpTourProps {
  commands: Command[];
  run: boolean;
  setRun: (run: boolean) => void;
}

export default function HelpTour({ commands, run, setRun }: HelpTourProps) {
  const steps = [
    {
      target: '#add-command',
      content: 'Click here to add a command to your stack.',
      disableBeacon: true,
    },
    ...(commands?.some((cmd) => cmd.tag)
      ? [
          {
            target: '#tag-tree',
            content: 'Browse your commands by tags.',
            disableBeacon: true,
          },
        ]
      : []),
    ...(commands?.some((cmd) => cmd.favourite)
      ? [
          {
            target: '#favourites-nav',
            content: 'Filter by your favourite commands.',
            disableBeacon: true,
          },
        ]
      : []),
  ];

  const handleJoyrideCallback = (data: CallBackProps) => {
    const { status } = data;

    if (status === STATUS.FINISHED || status === STATUS.SKIPPED) {
      setRun(false);
    }
  };

  return (
    <Joyride
      steps={steps}
      run={run}
      callback={handleJoyrideCallback}
      tooltipComponent={TourTooltip}
      continuous
      floaterProps={{
        hideArrow: true,
      }}
    />
  );
}
