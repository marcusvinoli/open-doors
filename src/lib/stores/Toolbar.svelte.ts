import type { Toolbar, ToolbarGroupType } from "$lib/components/global/toolbar/Toolbar";

let _toolbarItems: ToolbarGroupType[] = $state([]);

export function toolbarItems() {
    return _toolbarItems;
}

export function addToolbarItem(item: ToolbarGroupType) {
    _toolbarItems.push(item);
};

export function clearToolbar() {
    _toolbarItems = [];
}
