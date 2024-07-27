import type { TreeItem } from "./Tree";

export interface ProjectManifest {
    name: string,
    prefix: string,
    separator: string,
}

export interface Project {
    manifest: ProjectManifest,
    tree: TreeItem,
}
