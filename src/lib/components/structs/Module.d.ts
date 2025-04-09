import type { Links } from "./Links";

export interface Module {
    path: string,
    manifest: ModuleManifest,
    template: Template,
    baselines: Baseline[],
    links: Links,
}
