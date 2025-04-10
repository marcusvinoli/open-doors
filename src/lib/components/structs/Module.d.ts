import type { Links } from "./Links";

export interface Module {
    inboundLinks(inboundLinks: any, id: number): unknown;
    path: string,
    manifest: ModuleManifest,
    template: Template,
    baselines: Baseline[],
    links: Links,
}
