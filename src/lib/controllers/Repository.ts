import type { Repository, RepositoryManifest } from "$lib/components/structs/Repo";
import { invoke } from "@tauri-apps/api";

export function saveRepoInformation(repos: any) {
    localStorage.setItem('repository', JSON.stringify(repos));
}

export async function createRepository(man: RepositoryManifest) {
    return invoke('create_repo', {repo: man})
}

export async function reloadRepository() {
    let openRepo = JSON.parse(localStorage.getItem('repository') as string) as Repository;
    invoke('read_repo', {path: openRepo.path})
        .then((repo) => {
            saveRepoInformation(repo)
        })
        .catch((err) => {
            console.log(err)
        })
}
