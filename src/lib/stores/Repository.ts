import { writable } from "svelte/store";
import type { Repository } from "$lib/components/structs/Repo";

export const repository = writable<Repository | null>(null);
