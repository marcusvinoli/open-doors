import type { Repository, RepositoryManifest } from "$lib/components/structs/Repo";
import { invoke } from "@tauri-apps/api";
import { repository } from "../../routes/store";

export function saveRepoInformation(repo: Repository) {
    repository.set(repo);
    localStorage.setItem('repository', JSON.stringify(repo));
}

export function loadRepoInformation(): Repository {
    return JSON.parse(localStorage.getItem('repository') as string) as Repository;
}

export async function openRepository(path: string) {
    return invoke('read_repo', {path: path})
        .then((repo) => {
            saveRepoInformation(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function cloneRepository(path: string, remote: string) {
    return invoke('clone_repo', {path: path, remote: remote})
        .then((repo) => {
            saveRepoInformation(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function createRepository(path: string, man: RepositoryManifest) {
    return invoke('create_repo', {man: man, path: path})
        .then((repo) => {
            saveRepoInformation(repo as Repository);
        })
        .catch((err) => {
            console.log(err);
        })
}

export async function reloadRepository() {
    invoke('read_repo', {path: loadRepoInformation().path})
        .then((repo) => {
            saveRepoInformation(repo as Repository)
        })
        .catch((err) => {
            console.log(err)
        })
}
