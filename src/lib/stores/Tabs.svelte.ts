import { goto } from "$app/navigation";

import type { TabData } from "$lib/components/global/tabs/TabData";

let _tabs : TabData[] = $state([]);
let _activeTab: string = $state("/");

export function tabs() {
    return _tabs;
}

export function activeTab() {
    return _activeTab;
}

export function setActiveTab(tabPath: string) {
    if(_tabs.findIndex((tab) => {return (tab.path === tabPath)}) < 0) {
        return;
    }
    _activeTab = tabPath;
}

export function addTab(title: string, icon: string, path: string, badge: string | null = null, onClose?: () => void) {
    let newTab: TabData = {
        title: title,
        icon: icon,
        path: path,
        badge: badge,
        onClose: onClose,
    }

    let newTabs: TabData[] = _tabs;
    if(newTabs.findIndex((tab) => {return (tab.path === newTab.path)}) < 0) {
        newTabs.push(newTab);
    }
    _tabs = newTabs;
    _activeTab = path;
}

export function closeTab(path: string) {
    let newActive = _activeTab;
    let newTabs: TabData[] = [..._tabs];

    const indexToRemove = newTabs.findIndex((tab) => {
        return (tab.path === path);
    });

    if(indexToRemove < 0) {
        return;
    }

    if(newTabs[indexToRemove].path === newActive) {
        if (indexToRemove === 0) {
            newActive = (newTabs[indexToRemove+1]) ? newTabs[indexToRemove+1].path : "/";
        } else {
            newActive = newTabs[indexToRemove-1].path;
        }
    }
    const callback = _tabs[indexToRemove].onClose;
    if (callback) {
        callback();
    }
    newTabs.splice(indexToRemove, 1);
    if (_activeTab === path) {
        goto(newActive);
    }
    _tabs = newTabs;

}

export function clearTabs() {
    _tabs = [];
}

export function openTab(path: string) {
    if (_activeTab === path) {
        return;
    }
    goto(path)
    _activeTab = path;
}
