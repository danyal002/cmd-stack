import { invoke } from '@tauri-apps/api/core';
import { Button } from '../ui/button';
import { open, save } from '@tauri-apps/plugin-dialog';
import { toast } from '@/hooks/use-toast';
import { useCommands } from '@/use-command';

export function ImportButton() {
  const [, refreshCommands] = useCommands();

  return (
    <Button
      variant="outline"
      className="w-28"
      onClick={async () => {
        const path = await open({
          multiple: false,
          directory: false,
          filters: [
            {
              name: 'Commands',
              extensions: ['json'],
            },
          ],
        });

        if (!path) return;

        invoke('import_data', {
          path,
        })
          .then((res) => {
            toast({
              title: `Imported ${res} commands ✅`,
            });
            refreshCommands();
          })
          .catch((error) => {
            console.error(error);
            toast({
              title: `An error occurred whilst importing commands ❌`,
            });
          });
      }}
    >
      Choose JSON
    </Button>
  );
}

export function ExportButton() {
  return (
    <Button
      variant="outline"
      className="w-28"
      onClick={async () => {
        const path = await save({
          filters: [
            {
              name: 'Commands',
              extensions: ['json'],
            },
          ],
          defaultPath: 'cmdstack-export.json',
        });

        if (!path) return;

        await invoke('export_data', {
          path,
        })
          .then(() => {
            toast({
              title: 'Exported CmdStack to JSON ✅',
            });
          })
          .catch((error) => {
            console.error(error);
            toast({
              title: `An error occurred whilst exporting commands ❌`,
            });
          });
      }}
    >
      Save JSON
    </Button>
  );
}
