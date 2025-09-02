import { app } from "$lib/stores/AppState.svelte";
import { invoke } from "@tauri-apps/api";
import { loadAuthorInformation } from "./User";
//TODO: In the future, the store "repository" will be deprecated by "app" from "AppState"
import { repository, setRepository } from "$lib/stores/Repository.svelte";

import type { Repository } from "$lib/components/structs/Repo";

export function saveRepository(repo: Repository) {
	localStorage.setItem('repository', JSON.stringify(repo));
	app.repository = repo;
	setRepository(repo); //TODO: To be removed.
}

export function loadRepository() {
	let repo = JSON.parse(localStorage.getItem('repository') as string) as Repository;
	if (repo) {
		saveRepository(repo);
		loadAuthorInformation();
		return true;
	}
	return false;
}

export async function openRepository(path: string) {
	return invoke('read_repository', {path: path})
		.then((repo) => {
			saveRepository(repo as Repository);
		})
		.catch((err) => {
			console.log(err);
		})
}

export async function cloneRepository(path: string, remote: string) {
	return invoke('clone_repository', {path: path, remote: remote})
		.then((repo) => {
			saveRepository(repo as Repository);
		})
		.catch((err) => {
			console.log(err);
		})
}

export async function createRepository(path: string, name: string, remote: string | null) {
	return invoke('create_repository', {path: path, name: name, remote: remote})
		.then((repo) => {
			saveRepository(repo as Repository);
		})
		.catch((err) => {
			console.log(err);
		})
}

export async function reloadRepository() {
	let repo = repository();
	if (!repo) {
		if (loadRepository()) {
			reloadRepository()
			return;
		}
		return;
	}

	return invoke('read_repository', {path: repo.tree.path})
		.then((repo) => {
			saveRepository(repo as Repository)
		})
		.catch((err) => {
			console.log(err)
		})
}

export function clearRepositoryInformation() {
	app.repository = null;
	setRepository(null); //TODO: To be removed.
}
