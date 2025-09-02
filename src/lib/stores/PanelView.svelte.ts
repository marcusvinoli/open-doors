import { repository } from "$lib/stores/Repository.svelte";
import { listRelatives } from "$lib/utils/lists";

import type { TreeItem } from "$lib/components/structs/Tree";

let _treeHistory : TreeItem[] = $state([repository()?.tree]);
let _currentItem : TreeItem | null = $derived.by(() => _treeHistory.at(_treeHistory.length - 1) ?? repository()?.tree);

function findMatchingItem(path: string, item: TreeItem): TreeItem | null {
  if (item.path === path) {
    return item;
  }
  for (const child of item.children) {
    const found = findMatchingItem(path, child);
    if (found) {
      return found;
    }
  }
  return null;
}

function refreshTreeHistory() {
    let repo = repository();
    if (!repo) {
      return;
    }
    let treeHist = _treeHistory.map(historyItem => {
        const matchingItem = findMatchingItem(historyItem.path, repo.tree);
        if (matchingItem) {
        return matchingItem;
        } else {
        return { ...historyItem, children: [] };
        }
    });
    _treeHistory = treeHist;
}

export function currentItem() {
    return _currentItem;
}

export function treeHistory() {
    return _treeHistory;
}

export function goBack() {
    refreshTreeHistory()
    if (_treeHistory.length > 1) {
        _treeHistory.pop();
    }
}

export function goTo(item: TreeItem){
    let repo = repository();
    if (!repo) {
        return;
    }
    if (item.path === repo.tree.path) {
        _treeHistory = [ repo.tree ];
    }
    item = findMatchingItem(item.path, repo.tree) ?? item;
    let relatives = listRelatives(repo.tree, repo.tree, item)
    if (relatives.length === 0) {
        return;
    }
    _treeHistory = [...relatives, item]
}

export function goHome() {
    refreshTreeHistory();
    let home = _treeHistory.shift();
    if (!home) {
        return;
    }
    goTo(home);
}
