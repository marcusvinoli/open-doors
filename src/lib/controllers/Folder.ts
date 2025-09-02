import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree";

export async function createFolder(name: string, parent: TreeItem) {
    return invoke('create_folder', {name, parent})
        .then((folder) => {
            reloadRepository();
            return Promise.resolve(folder as TreeItem);
        })
        .catch((err) => {
            return Promise.reject(err);
        })
}

export async function readFolder(folder: TreeItem) : Promise<TreeItem> {
    return invoke('read_folder', {folder: folder});
}

export async function updateFolder(origin: string, destination: string) : Promise<TreeItem> {
    return invoke('update_folder', {origin: origin, destination: destination});
}

export async function deleteFolder(folder: TreeItem) : Promise<TreeItem> {
    return invoke('delete_folder', {path: folder.path});
}
