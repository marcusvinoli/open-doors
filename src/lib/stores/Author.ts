import { writable } from "svelte/store";
import type { Author } from "$lib/components/structs/Author";

export const author = writable<Author>({email: "", name: "", usename: null});
