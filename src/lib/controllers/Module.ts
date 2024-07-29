import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree"
import type { ModuleManifest, Module } from "$lib/components/structs/Module"

export function createModule(manifesf: ModuleManifest, parent: TreeItem) {
    return invoke('create_module', {man: manifesf, parent: parent})
        .then((module) => {
            reloadRepository();
            return module as Module;
        })
        .catch((err) => {
            console.log(err)
        })
}

export function readModule(module: TreeItem) {
    return invoke('read_module', {path: module.path})
        .then((mod) => {
            return mod as Module;
        })
}

export function updateModuleManifest(module: TreeItem, manifest: ModuleManifest) {
    return invoke('update_module', {path: module.path, man: manifest})
        .then((mod) => {
            return mod as Module;
        })
}

export function deleteModule(module: TreeItem) {
    return invoke('delete_module', {path: module.path})
}

export function readAddObjects(module: TreeItem) {

}

export function createDraftObject(module: TreeItem, objects: Object | Object[]) {

}

export function createObjects(module: TreeItem, object: Object | Object[]) {

}