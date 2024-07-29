import type { Author } from "$lib/components/structs/Author";
import { author } from "$lib/stores/Author";
import { repository } from "$lib/stores/Repository";
import { invoke } from "@tauri-apps/api";
import { get } from "svelte/store";

export function loadAuthorInformation() {
    let repo = get(repository);
    console.log("repo", repo);
    if(repo) {
        invoke('get_user', {path: repo.tree.path})
            .then(usr => {
                console.log("Urs", usr);
                author.set(usr as Author);
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
