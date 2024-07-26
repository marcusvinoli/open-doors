import type { ModuleManifest } from "$lib/components/structs/Module"
import type { Repository } from "$lib/components/structs/Repo"
import type { TreeItem } from "$lib/components/structs/Tree"
import { invoke } from "@tauri-apps/api";
import { loadRepoInformation } from "./Repository";
import { updateRepoTree } from "./Tree";

export function createModule(manifesf: ModuleManifest, parent: TreeItem) {
    return invoke('create_module', {path: loadRepoInformation().path, manifest: manifesf, parent: parent})
        .then((parentUpdate) => {
            updateRepoTree(parentUpdate as TreeItem)
            console.log(parentUpdate)
        })
        .catch((err) => {
            console.log(err)
        })
}