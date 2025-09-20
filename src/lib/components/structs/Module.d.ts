import type { Links } from "./Links";
import type { Template } from "./Template";
import type { Baseline } from "./Baseline";
import type { ModuleManifest } from "./ModuleManifest";
import type { View } from "./View";

export interface Module {
    path: string,
    manifest: ModuleManifest,
    template: Template,
    views: View[],
    baselines: Baseline[],
    links: Links,
}
