import type { Baseline } from "$lib/components/structs/Module";

export function getBaselineStatusIcon(status: string): string {
    if (status === "Current Baseline") {
        return "gravity-ui:circle-check-fill";
    } else if (status === "Work in Progress") {
        return "gravity-ui:circle";
    }
    return "gravity-ui:circle-exclamation-fill";
}

export function getBaselineStatus(baseline: Baseline, current: boolean = false): string {
    if (current && baseline.commit) {
        return "Current Baseline";
    } else if (baseline.commit) {
        return "Deprecated";
    }
    return "Work in Progress";
}

export function getBaselineStatusIconColor(status: string) {
    if (status === "Current Baseline") {
        return "#00802b";
    } else if (status === "Work in Progress") {
        return "#007acc";
    }
    return "#e6ac00";
}