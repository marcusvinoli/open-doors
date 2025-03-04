import { writable } from "svelte/store";
import type { View } from "../components/structs/View";

export const view = writable<View>({items: []});
