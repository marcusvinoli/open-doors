import { invoke } from "@tauri-apps/api";
import type { TreeItem } from "$lib/components/structs/Tree";

export function readTree(item: TreeItem) {
    invoke('read_tree', {path: item.path})
        .then((tree) => {
            return tree;
        })
        .catch((err) => {
            console.log(err)
        })
}
