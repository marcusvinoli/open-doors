import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree"
import type { ModuleManifest } from "$lib/components/structs/Module"

export function createModule(manifesf: ModuleManifest, parent: TreeItem) {
    return invoke('create_module', {man: manifesf, parent: parent})
        .then((module) => {
            reloadRepository();
            return module;
        })
        .catch((err) => {
            console.log(err)
        })
}
