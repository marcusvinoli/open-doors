import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree"
import type { ModuleManifest, Module } from "$lib/components/structs/Module"
import type { ObjectView, Object } from "$lib/components/structs/Object";
import type { Template } from "$lib/components/structs/Template";

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
    return readModuleFromPath(module.path)
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

export function readAllObjects(module: TreeItem) {

}

export function readModuleFromPath(path: string) {
    return invoke('read_module', {path: path})
        .then((mod) => {
            return mod as Module;
        })
}

export function createObject(modulePath: String, object: Object | ObjectView) {
    return invoke('create_object', {path: modulePath, object: object})
        .then((mod) => {
            return mod as Object;
        })
}

export function createDraftObject(modulePath: String, object: Object | ObjectView) {
    return invoke('create_draft_object', {path: modulePath, object: object as Object})
        .then((mod) => {
            return mod as Object;
        })
}

export function readObjects(modulePath: String) {
    return invoke('read_objects', {path: modulePath})
        .then((mods) => {
            return mods as Object[];
        })
}

export function readDraftObjects(modulePath: String) {
    return invoke('read_draft_objects', {path: modulePath})
        .then((mods) => {
            return mods as Object[];
        })
}

export function deleteObject(modulePath: String, id: number) {
    return invoke('delete_object', {path: modulePath, id: id})
}

export function saveTemplate(modulePath: String, template: Template) {
    return invoke('update_template', {path: modulePath, template: template})
}