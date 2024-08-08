import type { ObjectView, Link } from "./Object";
import type { Template } from "./Template";
export interface ModuleManifest {
    title: string,
    prefix: string,
    separator: string,
    description: string,
}

export interface Baseline {
    version: string,
    description: string,
    commit: string | null,
}

export interface Module {
    path: string,
    manifest: ModuleManifest,
    template: Template,
    baselines: Baseline[],
    inboundLinks: IHash,
}

export interface ModuleView extends Module {
    objects: ObjectView[] = [], 
    links: Link[] = [],
}
