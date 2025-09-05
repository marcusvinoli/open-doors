export const prerender = false;

import { app } from '$lib/stores/AppState.svelte';
import { addTab } from '$lib/stores/Tabs.svelte';
import { loadModule } from '$lib/stores/AppState.svelte';

import type { PageLoad } from './$types';

function setupTab(url: string, mod: string, version?: string) {
    const name : string = mod.substring(app.repository!.tree.path.length);
    const baseline : string = version ?? 'current';
    const icon: string = 'gravity-ui:layout-header-cells-large-fill';
    addTab(name, icon, url, baseline);
}

export const load: PageLoad = ({ params, url }) => {
    const mod: string = params.mod;
    const version: string = 'current';
    return loadModule(mod, version)
        .then(() => {
            setupTab(url.pathname, mod);
        })
};
