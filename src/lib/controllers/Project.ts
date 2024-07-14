import { invoke } from "@tauri-apps/api";
import { loadRepoInformation } from "./Repository";
import type { Project, ProjectManifest } from "$lib/components/structs/Project";
import type { TreeItem } from "$lib/components/structs/Tree";
import { repository } from "../../routes/store";

function updateRepoTree(newTree: TreeItem) {
    let repo = loadRepoInformation();
    let parentName = newTree.name;
    let replaceParent = (treeItem: TreeItem, parentName: string, modifiedParent: TreeItem): boolean => {
        let found = false;

        let visit = (node: TreeItem) => {
            if (node.name === parentName) {
                node.children = modifiedParent.children;
                found = true;
            }
        };

        let visitAll = (treeItem: TreeItem, parentName: string, visit: (node: TreeItem) => void) => {
            visit(treeItem);
            for (let child of treeItem.children) {
                visitAll(child, parentName, visit);
            }
        };

        visitAll(treeItem, parentName, visit);
        return found;
    };
    replaceParent(repo.structure, parentName, newTree);
    repository.set(repo);
    console.log(repo);
}

export async function updateTree(project: Project, parent: TreeItem) {
    invoke('update_structure_file', {child: project.tree, parent: parent})
        .then((parentUpdate) => {
            updateRepoTree(parentUpdate as TreeItem)
            console.log(parentUpdate)
        })
        .catch((err) => {
            console.log(err)
        })
}

export async function createProject(manifest: ProjectManifest, parent: TreeItem) {
    invoke('create_project', {man: manifest, path: loadRepoInformation().path})
        .then((prj) => {
            updateTree(prj as Project, parent);
            console.log(prj);
        })
        .catch((err) => {
            console.log(err);
        })
}