import type TreeItem from "./Tree";

export interface RepositoryManifest {
    name: string;
}

export interface Repository {
    structure: TreeItem;
    tree: RepositoryManifest;
}