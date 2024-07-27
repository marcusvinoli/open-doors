import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree";

export async function createFolder(name: string, parent: TreeItem) {
    invoke('create_folder', {name: name, parent: parent})
        .then((folder) => {
            reloadRepository();
            return folder;
        })
        .catch((err) => {
            console.log(err)
        })
}

export async function readFolder(folder: TreeItem) {
    invoke('read_folder', {folder: folder})
}