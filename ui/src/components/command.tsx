'use client';

import { cmdStackIcon, cmdStackIconWithText } from '@/components/cmdStackIcon';
import { CommandDisplay } from '@/components/command-display/command-display';
import { CommandList } from '@/components/command-list';
import { Nav } from '@/components/nav';
import { TagTree } from '@/components/tag-tree';
import { cn } from '@/lib/utils';
import { Command } from '@/types/command';
import { useCommand } from '@/use-command';
import { File, ListFilter, Star, Tags } from 'lucide-react';
import { useState } from 'react';
import { AddDialog } from './add-dialog';
import { SearchForm } from './search-form';
import { SettingsDialog } from './settings-dialog';
import { Badge } from './ui/badge';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './ui/resizable';
import { ScrollArea } from './ui/scroll-area';
import { Separator } from './ui/separator';
import { TooltipProvider } from './ui/tooltip';

interface MainCommandPageProps {
  commands: Command[];
  defaultLayout: number[] | undefined;
  defaultCollapsed?: boolean;
  navCollapsedSize: number;
}

export function MainCommandPage({
  commands,
  defaultLayout = [20, 32, 48],
  defaultCollapsed = false,
  navCollapsedSize,
}: MainCommandPageProps) {
  const [favouriteFilter, setFavouriteFilter] = useState(false);
  const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
  const [selectedTagId, setSelectedTagId] = useState<string | undefined>();

  const [command] = useCommand();

  const handleClickTag = (tagId: string | undefined) => {
    setSelectedTagId(tagId);
  };

  const handleDeselectTag = () => {
    setSelectedTagId(undefined);
  };

  const tagFilteredCommands = selectedTagId
    ? commands.filter(
        (command) => command.tag && command.tag.startsWith(selectedTagId),
      )
    : commands;

  return (
    <TooltipProvider delayDuration={0}>
      <ResizablePanelGroup
        direction="horizontal"
        onLayout={(sizes: number[]) => {
          document.cookie = `react-resizable-panels:layout:mail=${JSON.stringify(
            sizes,
          )}`;
        }}
        className="h-full items-stretch"
      >
        <ResizablePanel
          defaultSize={defaultLayout[0]}
          collapsedSize={navCollapsedSize}
          collapsible={true}
          minSize={15}
          maxSize={20}
          onCollapse={() => {
            setIsCollapsed(true);
            document.cookie = `react-resizable-panels:collapsed=${JSON.stringify(
              true,
            )}`;
          }}
          onResize={() => {
            setIsCollapsed(false);
            document.cookie = `react-resizable-panels:collapsed=${JSON.stringify(
              false,
            )}`;
          }}
          className={cn(
            isCollapsed &&
              'min-w-[50px] transition-all duration-300 ease-in-out',
            'flex h-screen flex-col',
          )}
        >
          <div
            className={cn(
              'flex h-[52px] items-center justify-center',
              isCollapsed ? 'h-[52px]' : 'px-2',
            )}
          >
            <div className={cn('flex gap-2 [&_svg]:h-8')}>
              {isCollapsed ? cmdStackIcon : cmdStackIconWithText}
            </div>
          </div>
          <Separator />
          <Nav
            isCollapsed={isCollapsed}
            links={[
              {
                title: 'Commands',
                label: tagFilteredCommands.length.toString(),
                icon: File,
                variant: !favouriteFilter ? 'default' : 'ghost',
                onClick() {
                  setFavouriteFilter(false);
                },
              },
              {
                title: 'Favourites',
                label: tagFilteredCommands
                  .filter((item) => item.favourite)
                  .length.toString(),
                icon: Star,
                variant: favouriteFilter ? 'default' : 'ghost',
                onClick() {
                  setFavouriteFilter(true);
                },
              },
            ]}
          />
          <Separator />
          <div onClick={handleDeselectTag}>
            <Nav
              isCollapsed={isCollapsed}
              links={[
                {
                  title: 'All Tags',
                  icon: Tags,
                  variant: 'ghost',
                  onClick() {},
                },
              ]}
            />
          </div>
          {!isCollapsed && (
            <ScrollArea className="flex-1">
              <TagTree
                commands={commands}
                selectedTagId={selectedTagId}
                handleSelectedTagIdChange={handleClickTag}
              />
            </ScrollArea>
          )}
          <Separator />
          <SettingsDialog isCollapsed={isCollapsed} />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel
          defaultSize={defaultLayout[1]}
          minSize={30}
          className="flex h-screen flex-col min-w-[290px]"
        >
          <div className="flex items-center pl-4 pr-2 py-2">
            <h1 className="text-xl font-bold">Commands</h1>
            <div className="ml-auto">
              <AddDialog />
            </div>
          </div>
          <Separator />
          <div className="bg-background/95 p-4 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <SearchForm />
          </div>
          {selectedTagId && (
            <div className="flex gap-2 px-4 mb-2 items-center">
              <ListFilter size={12} />
              <Badge variant="secondary">{selectedTagId}</Badge>
            </div>
          )}
          <ScrollArea className="flex-1">
            <CommandList
              items={
                favouriteFilter
                  ? tagFilteredCommands.filter((item) => item.favourite)
                  : tagFilteredCommands
              }
            />
          </ScrollArea>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={defaultLayout[2]} minSize={30}>
          <CommandDisplay
            command={
              commands.find((item) => item.id === command.selected) || null
            }
          />
        </ResizablePanel>
      </ResizablePanelGroup>
    </TooltipProvider>
  );
}
