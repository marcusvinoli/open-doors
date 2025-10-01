
import { app } from "$lib/stores/AppState.svelte";
import { User } from "$lib/components/structs/User";
import { invoke } from "@tauri-apps/api";

export function loadAuthorInformation() : Promise<void> {
    return new Promise((resolve, reject) => {
        if (!app.repository) {
            return reject(new Error('No repository is opened.'))
        }     
        invoke('get_user', {path: app.repository.tree.path})
            .then(usr => {
                console.log(usr);
                app.user = User.fromString(usr as string);
                resolve();
            })
            .catch(err => {
                console.error(err);
                reject(err);
            })
    })
}

export function clearAuthorInformation() {
    app.user = null;
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
