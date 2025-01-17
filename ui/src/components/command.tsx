'use client';

import { useState } from 'react';
import { File, Search, Settings, Star, Tags } from 'lucide-react';

import { cn } from '@/lib/utils';
import { Input } from './ui/input';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './ui/resizable';
import { Separator } from './ui/separator';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { TooltipProvider } from './ui/tooltip';
import { Badge } from './ui/badge';
import { CommandDisplay } from '@/components/command-display';
import { CommandList } from '@/components/command-list';
import { TagTree } from '@/components/tag-tree';
import { Nav } from '@/components/nav';
import { useCommand } from '@/use-command';
import { Command } from '@/types/command';
import { cmdStackIcon } from '@/components/cmdStackIcon';
import { AddDialog } from './add-dialog';
import { ScrollArea } from './ui/scroll-area';

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
          )}
        >
          <div
            className={cn(
              'flex h-[52px] items-center justify-center',
              isCollapsed ? 'h-[52px]' : 'px-2',
            )}
          >
            <div
              className={cn('flex items-center gap-2 [&_svg]:h-6 [&_svg]:w-6')}
            >
              {cmdStackIcon}
              {!isCollapsed && (
                <h1 className="text-base font-normal">CmdStack</h1>
              )}
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
                variant: 'default',
              },
              {
                title: 'Favorites',
                label: tagFilteredCommands
                  .filter((item) => item.favourite)
                  .length.toString(),
                icon: Star,
                variant: 'ghost',
              },
            ]}
          />
          <Separator />
          <Nav
            isCollapsed={isCollapsed}
            links={[
              {
                title: 'Settings',
                icon: Settings,
                variant: 'ghost',
              },
            ]}
          />
          <Separator />
          <AddDialog />
          <Separator />
          <div onClick={handleDeselectTag}>
            <Nav
              isCollapsed={isCollapsed}
              links={[
                {
                  title: 'All Tags',
                  icon: Tags,
                  variant: 'ghost',
                },
              ]}
            />
          </div>
          {!isCollapsed && (
            <ScrollArea className="h-[calc(100vh-284px)]">
              <TagTree
                commands={commands}
                selectedTagId={selectedTagId}
                handleSelectedTagIdChange={handleClickTag}
              />
            </ScrollArea>
          )}
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel
          defaultSize={defaultLayout[1]}
          minSize={30}
          className="min-w-[290px]"
        >
          <Tabs defaultValue="all">
            <div className="flex items-center px-4 py-2">
              <h1 className="text-xl font-bold">Commands</h1>
              <TabsList className="ml-auto">
                <TabsTrigger
                  value="all"
                  className="text-zinc-600 dark:text-zinc-200"
                >
                  All
                </TabsTrigger>
                <TabsTrigger
                  value="favourites"
                  className="text-zinc-600 dark:text-zinc-200"
                >
                  Favourites
                </TabsTrigger>
              </TabsList>
            </div>
            <Separator />
            <div className="bg-background/95 p-4 backdrop-blur supports-[backdrop-filter]:bg-background/60">
              <form>
                <div className="relative">
                  <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
                  <Input placeholder="Search" className="pl-8" />
                </div>
              </form>
            </div>
            {selectedTagId && (
              <div className="flex gap-2 px-4 mb-2">
                <Badge variant="secondary">{selectedTagId}</Badge>
              </div>
            )}
            <TabsContent value="all" className="m-0">
              <CommandList items={tagFilteredCommands} />
            </TabsContent>
            <TabsContent value="favourites" className="m-0">
              <CommandList
                items={tagFilteredCommands.filter((item) => item.favourite)}
              />
            </TabsContent>
          </Tabs>
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
