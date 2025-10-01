import type { TreeItem } from "./Tree";

export interface RepositoryManifest {
    name: string;
    apiVersion: string,
}

export interface Repository {
    tree: TreeItem;
    manifest: RepositoryManifest;
}