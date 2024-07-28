import { writable } from "svelte/store";
import type { TabData } from "$lib/components/global/tabs/TabData";

export const tabs = writable<TabData[]>([]);

export const newTab = writable<TabData | null>(null);
