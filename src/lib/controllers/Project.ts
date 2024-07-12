import { invoke } from "@tauri-apps/api";
import { loadRepoInformation } from "./Repository";
import type { Project, ProjectManifest } from "$lib/components/structs/Project";
import type { TreeItem } from "$lib/components/structs/Tree";

export async function updateTree(project: Project, parent: TreeItem) {
    invoke('update_structure_file', {newTree: project.tree, parent: parent})
        .then(() => {
        })
        .catch((err) => {
            console.log(err)
        })
}

export async function createProject(manifest: ProjectManifest, parent: TreeItem) {
    invoke('create_project', {man: manifest, path: loadRepoInformation().path})
        .then((prj) => {
            updateTree(prj as Project, parent);
        })
        .catch((err) => {
            console.log(err);
        })
}