import type Author from "$lib/components/structs/Author";
import { User } from "$lib/components/structs/User";
import { user } from "$lib/stores/User";
import { repository } from "$lib/stores/Repository";
import { invoke } from "@tauri-apps/api";
import { get } from "svelte/store";

export function loadAuthorInformation() {
    let repo = get(repository);
    if(repo) {
        invoke('get_user', {path: repo.tree.path})
            .then(usr => {
                user.set(User.fromString(usr as string));
            })
            .catch(err => {
                console.log(err);
            })
    }
}

export function getFirstAndLastName(fullName: string): string {
    const nameParts = fullName.trim().split(/\s+/);
    
    if (nameParts.length === 0) {
        return '';
    }
    
    const firstName = nameParts[0];
    const lastName = nameParts[nameParts.length - 1];
    
    return `${firstName} ${lastName}`;
}
