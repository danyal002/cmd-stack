import { MainCommandPage } from '@/components/command';
import { Toaster } from './components/ui/toaster';
import { useCommands } from './use-command';

export default function CommandPage() {
  const [commands, _] = useCommands();

  const defaultLayout = undefined;
  const defaultCollapsed = undefined;

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
      </div>
    </>
  );
}
