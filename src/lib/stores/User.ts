import { writable } from "svelte/store";
import { User } from "$lib/components/structs/User"

export const user = writable<User>(new User("", ""));
