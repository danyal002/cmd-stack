'use client';

import { useMemo } from 'react';

import { Command } from '@/types/command';
import { TreeView, TreeDataItem } from './ui/tree-view';

interface TagTreeProps {
  commands: Command[];
  selectedTagId: string | undefined;
  handleSelectedTagIdChange: (tag: string | undefined) => void;
}

export function TagTree({
  commands,
  selectedTagId,
  handleSelectedTagIdChange,
}: TagTreeProps) {
  const buildTagTree = (commands: Command[]): TreeDataItem[] => {
    interface TagNode {
      id: string;
      name: string;
      children: Record<string, TagNode>;
      onClick: () => void;
    }

    const root: Record<string, TagNode> = {};

    commands.forEach((command) => {
      if (!command.tag) return;
      const tagPath = command.tag.split('/');
      let current = root;

      tagPath.forEach((tagPart, index) => {
        if (!current[tagPart]) {
          const tagId = tagPath.slice(0, index + 1).join('/');
          current[tagPart] = {
            id: tagId,
            name: tagPart,
            children: {},
            onClick: () => {
              handleSelectedTagIdChange(tagId);
            },
          };
        }
        current = current[tagPart].children;
      });
    });

    const toTreeDataItems = (node: Record<string, TagNode>): TreeDataItem[] => {
      return Object.values(node).map(({ id, name, children, onClick }) => {
        const treeDataItem: TreeDataItem = {
          id,
          name,
          onClick,
        };

        const childItems = toTreeDataItems(children);
        if (childItems.length > 0) {
          treeDataItem.children = childItems;
        }

        return treeDataItem;
      });
    };

    return toTreeDataItems(root);
  };

  const tagData = useMemo(() => buildTagTree(commands), [commands]);
  return (
    <TreeView
      className="pl-2"
      data={tagData}
      initialSelectedItemId={selectedTagId}
      selectedItemId={selectedTagId}
      handleSelectedItemIdChange={handleSelectedTagIdChange}
    />
  );
}
