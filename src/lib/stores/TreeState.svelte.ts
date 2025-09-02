import type { TreeItem } from "$lib/components/structs/Tree";

let _treeState = new Map();

export function setTreeState(item: TreeItem, state: boolean) {
    _treeState.set(item.path, state);
    return state;
}

export function getTreeState(item: TreeItem) {
    if (_treeState.has(item.path)) {
        return _treeState.get(item.path);
    }
    _treeState.set(item.path, false);
    return false;
}
