import { goto } from "$app/navigation";
import { get, writable } from "svelte/store";
import type { TabData } from "$lib/components/global/tabs/TabData";

export const tabs = writable<TabData[]>([]);
export const activeTab = writable<string>("/");

export const addTab = (title: string, icon: string, path: string, badge: string | null = null) => {
    let newTab: TabData = {
        title: title,
        icon: icon,
        path: path,
        badge: badge,
    }

    let newTabs: TabData[] = get(tabs);
    if(newTabs.findIndex((tab) => {return (tab.path === newTab.path)}) < 0) {
        newTabs.push(newTab);
    }
    tabs.set(newTabs);
    activeTab.set(path);
}

export const closeTab = (path: string) => {
    let active = get(activeTab);

    let newTabs: TabData[] = get(tabs);

    const indexToRemove = newTabs.findIndex((tab) => {
        return(tab.path === path);
    });

    if(indexToRemove < 0) {
        return;
    }

    if(newTabs[indexToRemove].path === active) {
        if (indexToRemove === 0) {
            active = newTabs[indexToRemove+1] ? newTabs[indexToRemove+1].path : "/";
        } else {
            active = (newTabs[indexToRemove-1]) ? newTabs[indexToRemove-1].path : "/";
        }
    }

    newTabs.splice(indexToRemove, 1);

    tabs.set(newTabs);
    
    if (get(activeTab) !== active) {
        goto(active);
    }
}

export const clearTabs = () => {
    tabs.set([]);
}

export const openTab = (path: string) => {
    goto(path);
}
