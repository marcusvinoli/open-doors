import type TreeItem from "./Tree";

export interface RepositoryManifest {
    name: string;
}

export interface Repository {
    tree: TreeItem;
    manifest: RepositoryManifest;
}