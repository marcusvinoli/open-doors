import { writable } from "svelte/store";
import type { Toolbar, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";

export const toolbarItems = writable<ToolbarGroupType []>([]);

export const addToolbarItem = (item: ToolbarGroupType) => {
    toolbarItems.update(tb => {
        tb = [...tb, item]
        return tb;
    })
};

export const clearToolbar = () => {
    toolbarItems.set([])
}
