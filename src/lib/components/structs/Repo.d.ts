import type TreeItem from "./Tree";

export interface RepositoryManifest {
    name: string;
    remote: string | null;
}

export interface Repository {
    path: string;
    structure: TreeItem;
    manifest: RepositoryManifest;
}