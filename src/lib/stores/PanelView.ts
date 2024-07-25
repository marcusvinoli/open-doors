import { get, writable } from "svelte/store";
import type { TreeItem } from "$lib/components/structs/Tree";
import { listRelatives } from "$lib/utils/lists";
import { repository } from "../../routes/store";

export const treeHistory = writable<TreeItem[]>([get(repository)?.structure]);
export const currentItem = writable<TreeItem | null>(get(repository)?.structure);

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
        th = listRelatives(repo?.structure, repo?.structure, item);
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
