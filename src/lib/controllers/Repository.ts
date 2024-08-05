import { invoke } from "@tauri-apps/api";
import { repository } from "$lib/stores/Repository";
import type { Repository } from "$lib/components/structs/Repo";
import { loadAuthorInformation } from "./User";

export function saveRepository(repo: Repository) {
    localStorage.setItem('repository', JSON.stringify(repo));
    repository.set(repo);
}

export function loadRepository(): Repository {
    let repo = JSON.parse(localStorage.getItem('repository') as string) as Repository;
    if (repo) {
        loadAuthorInformation();
        saveRepository(repo);
    }
    return repo;
}

export async function openRepository(path: string) {
    return invoke('read_repo', {path: path})
        .then((repo) => {
            saveRepository(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function cloneRepository(path: string, remote: string) {
    return invoke('clone_repo', {path: path, remote: remote})
        .then((repo) => {
            saveRepository(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function createRepository(path: string, name: string, remote: string | null) {
    return invoke('create_repo', {path: path, name: name, remote: remote})
        .then((repo) => {
            saveRepository(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function reloadRepository() {
    invoke('read_repo', {path: loadRepository().tree.path})
        .then((repo) => {
            saveRepository(repo as Repository)
        })
        .catch((err) => {
            console.log(err)
        })
}
