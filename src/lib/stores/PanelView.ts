import { get, writable } from "svelte/store";
import type { TreeItem } from "$lib/components/structs/Tree";
import { listRelatives } from "$lib/utils/lists";
import { repository } from "$lib/stores/Repository";

export const treeHistory = writable<TreeItem[]>([get(repository)?.tree]);
export const currentItem = writable<TreeItem | null>(get(repository)?.tree);

function updateTreeHistory(newRepository: TreeItem, treeHistory: TreeItem[]): TreeItem[] {
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
  
    return treeHistory.map(historyItem => {
      const matchingItem = findMatchingItem(historyItem.path, newRepository);
      if (matchingItem) {
        return matchingItem;
      } else {
        return { ...historyItem, children: [] };
      }
    });
}

repository.subscribe(repo => {
    if (repo) {
        let currentHistory = get(treeHistory);
        let newHistory: TreeItem[];
        newHistory = updateTreeHistory(repo.tree, currentHistory);
        treeHistory.set(newHistory);
    }
})

treeHistory.subscribe(th => {
    if(th.length > 1) {
        currentItem.set(th.at(th.length - 1)!);
        return;
    }
    currentItem.set(th[0]);
})

export const goBack = () => {
    treeHistory.update((th) => {
        if (th.length > 1) {
            th.pop();
            currentItem.set(th.at(th.length - 1)!);
        }
        return th;
    })
}

export const goIn = (item: TreeItem) => {
    treeHistory.update((th) => {
        const repo = get(repository);
        th = listRelatives(repo?.tree, repo?.tree, item);
        th.push(item);
        currentItem.set(item);
        console.log(item);
        return th;
    })
}

export const goHome = () => {
    treeHistory.update((th) => {
        let thCandidate = th.shift();
        if (thCandidate) {
            th = [thCandidate];
            currentItem.set(thCandidate);
        } else {
            th = [];
            currentItem.set(null);
        }
        return th;
    })
}
