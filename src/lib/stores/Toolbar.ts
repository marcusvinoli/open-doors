import { writable } from "svelte/store";
import type { Toolbar, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";

export const toolbarItems = writable<Toolbar>({items: []});

export const addToolbarItem = (item: ToolbarGroupType) => {
    toolbarItems.update(tb => {
        tb.items = [...tb.items, item]
        return tb;
    })
};

export const clearToolbar = () => {
    toolbarItems.update(tb => {
        tb.items = []
        return tb;
    })
}
