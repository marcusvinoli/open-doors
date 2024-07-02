export type ItemType = "folder" | "project" | "repository" | "module" | "other";

export interface Breadcrumb {
    type: ItemType,
    name: string,
}
