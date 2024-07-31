import { get, writable } from "svelte/store";
import type { ModuleView } from "$lib/components/structs/Module";
import type { ObjectView } from "$lib/components/structs/Object";
import type Module from "module";

export const modules = writable<ModuleView[]>([]);

export const addModule = (mod: Module) => {
    let mods = get(modules);
    let index = mods.findIndex((m) => {m.path == mod.path});
    if (index < 0) {
        let modview = mod as unknown as ModuleView;
        console.log(modview);
        mods.push(modview);
    }
}