import type { TreeItem } from "$lib/components/structs/Tree";
import type { TreeItemState } from "./TreeViewState";

export function convertToTreeItemState(item: TreeItem): TreeItemState {
  return {
    name: item.name,
    path: item.path,
    itemType: item.itemType,
    opened: true, 
    children: item.children.map(child => convertToTreeItemState(child))
  };
}

export function updateTreeItemState(item: TreeItem, state: TreeItemState): TreeItemState {
  const updatedState: TreeItemState = {
    name: item.name,
    path: item.path,
    itemType: item.itemType,
    opened: state.opened, // Preserve the opened state
    children: []
  };

  const stateChildrenMap = new Map<string, TreeItemState>();
  state.children.forEach(childState => {
    stateChildrenMap.set(`${childState.name}:${childState.path}`, childState);
  });

  item.children.forEach(childItem => {
    const key = `${childItem.name}:${childItem.path}`;
    const childState = stateChildrenMap.get(key);

    if (childState) {
      updatedState.children.push(updateTreeItemState(childItem, childState));
    } else {
      updatedState.children.push(convertToTreeItemState(childItem));
    }

    stateChildrenMap.delete(key);
  });

  return updatedState;
}
