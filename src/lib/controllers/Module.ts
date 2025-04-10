import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree"
import type { Module} from "$lib/components/structs/Module"
import type { ModuleManifest } from '$lib/components/structs/ModuleManifest';
import type { Baseline } from '$lib/components/structs/Baseline';
import type { Object } from "$lib/components/structs/Object";
import type { Template } from "$lib/components/structs/Template";

export async function createModule(manifesf: ModuleManifest, parent: TreeItem) {
    try {
        const module = await invoke('create_module', { man: manifesf, parent: parent });
        reloadRepository();
        return module as Module;
    } catch (err) {
        console.log(err);
    }
}

export function readModule(module: TreeItem) {
    return readModuleFromPath(module.path)
}

export async function updateModuleManifest(module: TreeItem, manifest: ModuleManifest) {
    const mod = await invoke('update_module', { path: module.path, man: manifest });
    return mod as Module;
}

export function deleteModule(module: TreeItem) {
    return invoke('delete_module', {path: module.path})
}

export function readAllObjects(module: TreeItem) {

}

export async function readModuleFromPath(path: string) {
    const mod = await invoke('read_module', { path: path });
    return mod as Module;
}

export function readBaselinedObject(path: string, id: number, version: string) {
    return invoke('read_object_from_baseline', {path, id, version});
 
}

export function readBaselinedObjects(path: string, version: string) {
    return invoke('read_objects_from_baseline', {path, version});
}

export async function createObject(modulePath: String, object: Object) {
    const mod = await invoke('create_object', { path: modulePath, object: object });
    return mod as Object;
}

export async function createDraftObject(modulePath: String, object: Object) {
    const mod = await invoke('create_draft_object', { path: modulePath, object: object as Object });
    return mod as Object;
}

export async function readObjects(modulePath: String) {
    const mods = await invoke('read_objects', { path: modulePath });
    return mods as Object[];
}

export async function readDraftObjects(modulePath: String) {
    const objs = await invoke('read_draft_objects', { path: modulePath });
    return objs as Object[];
}

export function deleteObject(modulePath: String, id: number) {
    return invoke('delete_object', {path: modulePath, id: id})
}

export function restoreObject(modulePath: String, id: number) {
	return invoke('restore_object', {path: modulePath, id: id})
}

export function saveTemplate(modulePath: String, template: Template) {
    return invoke('update_template', {path: modulePath, template: template})
}

export async function createBaseline(modulePath: String, baseline: Baseline) {
    return invoke('create_baseline', {path: modulePath, baseline})
}

export async function exportCSV(modulePath: String) {
    const folder = await open({
        directory: true,
        multiple: false
    });
    if (folder) {
        let path = folder as string;
        return invoke('export_csv', {modulePath: modulePath, filePath: path})
    }
}

export async function exportXlsx(modulePath: String) {
    const folder = await open({
        directory: true,
        multiple: false
    });
    if (folder) {
        let path = folder as string;
        return invoke('export_xlsx', {modulePath: modulePath, filePath: path})
    }
}
