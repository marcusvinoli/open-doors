import type { Links } from "./Links";
import type { Template } from "./Template";
import type { Baseline } from "./Baseline";
import type { ModuleManifest } from "./ModuleManifest";

export interface Module {
    path: string,
    manifest: ModuleManifest,
    template: Template,
    view: View[],
    baselines: Baseline[],
    links: Links,
}
