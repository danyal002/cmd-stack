import { MainCommandPage } from '@/components/command';
import { useEffect } from 'react';
import { Toaster } from './components/ui/toaster';
import { useCommands, useRefresh } from './use-command';

export default function CommandPage() {
  const [, refreshData] = useRefresh();
  const [commands] = useCommands();

  const defaultLayout = undefined;
  const defaultCollapsed = undefined;

  useEffect(() => {
    refreshData();
  }, []);

  return (
    <>
      <div className="hidden flex-col md:flex">
        <MainCommandPage
          commands={commands.state == 'hasData' ? commands.data : []}
          defaultLayout={defaultLayout}
          defaultCollapsed={defaultCollapsed}
          navCollapsedSize={4}
        />
        <Toaster />
      </div>
    </>
  );
}
