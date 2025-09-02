import type { Module } from "$lib/components/structs/Module";

let  _modules: Module[] = $state([]);

export function modules() {
    return _modules;
}

export const addModule = (mod: Module) => {
    let index = _modules.findIndex((m) => {m.path == mod.path});
    if (index < 0) {
        _modules.push(mod);
    }
}
