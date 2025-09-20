import { generateModuleKey } from "$lib/utils/module-utils";
import { readModuleFromPath, readObjects } from "$lib/controllers/Module";
import { buildTreeIndex } from "$lib/utils/index-tree.utils";

import type { Module } from "$lib/components/structs/Module";
import type { Object } from "$lib/components/structs/Object";
import type { AppState, ModuleState } from "$lib/components/structs/States";
import type { IndexItem } from "$lib/components/structs/IndexItem";
import type { Task } from "$lib/components/structs/Task";

import { defaultView } from "$lib/components/structs/View";

export let app : AppState = $state({
    repository: null,
    user: null,
    modules: new Map(),
    tasks: new Map<string, Task>(),
    linker: null,
    currentModule: null,
});

// TODO: Migrate here all core logic for application state (e.g.: load repo, load user...)
export async function loadModule(path: string, version?: string) {
    const modKey: string = generateModuleKey(app.repository!.tree.path, path, version);
    let objects: Object[] = [];
    let module: Module;
    let indexTree: IndexItem[] = [];
    return Promise.all([
        readModuleFromPath(path)
            .then((mod) => {
                module = mod as Module;
            }),
        readObjects(path)
            .then((objs) => {
                objects = [...objs as Object[]];
                indexTree = buildTreeIndex(objects);
            }),
    ]).then(() => {
        let newModule: ModuleState = {
            flags: {
                showNewBaselineDialog: false,
                showTemplateDialog: false,
                showRowsNumbering: false,
                showObjectDialog: false,
                showViewsDialog: false,
                showIndexPanel: false,
                showDeletions: false,
                showLinks: false,
                readOnly: false,
            },
            module: module,
            objects: [],
            currentView: defaultView,
            currentObject: null,
            indexTree: {
                tree: [],
                state: new Map<number, boolean>(),
                scroll: {
                    x: 0,
                    y: 0,
                },
            },
            filter: null,
            scroll: {
                x: 0,
                y: 0
            },
            context: new Map<string, string>(),
            tasks: new Map<string, Task>(),
        };
        const oldModule = app.modules.get(modKey);
        if (oldModule) {
            newModule = {...oldModule};
            newModule.module = module;
        }
        newModule.objects = objects;
        newModule.indexTree.tree = indexTree;
        app.modules.set(modKey, newModule);
        app.currentModule = newModule;
        app.currentModule.tasks = oldModule?.tasks ?? new Map<string, Task>();
    })
}

export function disposeModule(path: string, version?: string) {
    const modKey: string = generateModuleKey(app.repository!.tree.path, path, version);
    app.modules.delete(modKey);
}

export function setCurrentModule(path: string, version?: string) {
    const modKey: string = generateModuleKey(app.repository!.tree.path, path, version);
    app.currentModule = app.modules.get(modKey) ?? null;
    if (!app.currentModule) {
        loadModule(path, version);
    }
}
