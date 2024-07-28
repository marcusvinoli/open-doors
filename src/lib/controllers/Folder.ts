import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree";

export async function createFolder(name: string, parent: TreeItem) {
    return invoke('create_folder', {name: name, parent: parent})
        .then((folder) => {
            reloadRepository();
            return folder as TreeItem;
        })
        .catch((err) => {
            console.log(err)
        })
}

export async function readFolder(folder: TreeItem) {
    return invoke('read_folder', {folder: folder})
        .then((folder) => {
            return folder as TreeItem;
        })
}

export async function updateFolder(origin: string, destination: string) {
    return invoke('update_folder', {origin: origin, destination: destination})
        .then((folder) => {
            return folder as TreeItem;
        })
}

export async function deleteFolder(folder: TreeItem) {
    return invoke('delete_folder', {path: folder.path})
}
