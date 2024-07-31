import type { ObjectView } from "./Object";

export interface ModuleManifest {
    title: string,
    prefix: string,
    separator: string,
    description: string,
}

export type LinkType = "outbound" | "inbound";

export type Link = {
    to: string,
    from: string,
    type: LinkType,
}

export interface Baseline {
    version: string,
    description: string,
    commit: string | null,
}

export interface Module {
    path: string,
    manifest: ModuleManifest,
    template: any,
    baselines: Baseline[],
}

export interface ModuleView extends Module {
    objects: ObjectView[] = [], 
    links: Link[] = [],
}
