import { writable } from "svelte/store";
import type { Repository } from "$lib/components/structs/Repo";
import type { TabData } from "$lib/components/global/tabs/TabData";

export const repository = writable<Repository | null>(null);

export const showToolbar = writable<boolean>(false);

export const tabs = writable<TabData[]>([]);

export const newTab = writable<TabData | null>(null);