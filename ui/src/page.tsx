import { MainCommandPage } from '@/components/command';
import { Toaster } from './components/ui/toaster';
import { useCommands } from './use-command';
import { HelpCircle } from 'lucide-react';
import { useState } from 'react';
import HelpTour from './components/help-tour';

export default function CommandPage() {
  const [commands, _] = useCommands();

  const defaultLayout = undefined;
  const defaultCollapsed = undefined;

  const [run, setRun] = useState(false);

  return (
    <>
      <div className="hidden flex-col md:flex select-none">
        <MainCommandPage
          commands={commands ? commands : []}
          defaultLayout={defaultLayout}
          defaultCollapsed={defaultCollapsed}
          navCollapsedSize={4}
        />
        <Toaster />
        <div className="fixed bottom-4 right-4">
          <button
            className="bg-primary text-primary-foreground rounded-full p-2 shadow-lg hover:bg-primary/90"
            onClick={() => setRun(true)}
          >
            <HelpCircle size={24} />
          </button>
        </div>
        <HelpTour commands={commands} run={run} setRun={setRun} />
      </div>
    </>
  );
}
