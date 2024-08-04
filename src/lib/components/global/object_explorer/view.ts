import { writable } from "svelte/store";
import type { View } from "./viewStructs";

export const view = writable<View>({items: []});
