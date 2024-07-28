import { invoke } from "@tauri-apps/api";
import { reloadRepository } from "./Repository";
import type { TreeItem } from "$lib/components/structs/Tree";
import type { Project, ProjectManifest } from "$lib/components/structs/Project";

export async function createProject(manifest: ProjectManifest, parent: TreeItem) {
    return invoke('create_project', {path: parent.path, man: manifest})
        .then((prj) => {
            reloadRepository();
            return prj as Project;
        })
}

export async function readProject(project: TreeItem) {
    return invoke('read_project', {path: project.path})
        .then((prj) => {
            return prj as Project;
        })
}

export async function updateProject(project: TreeItem, man: ProjectManifest) {
    return invoke('update_project', {path: project.path, man: man})
}

export async function deleteProject(project: TreeItem) {
    return invoke('delete_project', {path: project.path})
}
