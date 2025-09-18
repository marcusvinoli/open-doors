import type { Toolbar, ToolbarItemType } from "$lib/components/global/toolbar/Toolbar";

let _toolbarItems: Toolbar = $state({ items: [] });

export function toolbar() {
    return _toolbarItems;
}

export function addToolbarItem(item: ToolbarItemType) {
    _toolbarItems.items.push(item);
};

export function setToolbar(toolbar: Toolbar) {
    _toolbarItems = toolbar;
}


export function clearToolbar() {
    _toolbarItems.items = [];
}
