export const prerender = false;

import { app } from '$lib/stores/AppState.svelte';
import { reloadRepository } from '$lib/controllers/Repository';
import { addTab, setActiveTab } from '$lib/stores/Tabs.svelte';
import { loadAuthorInformation } from '$lib/controllers/User';

function setupTab(url: string, mod: string, version?: string) {
    const name : string = mod.substring(app.repository!.tree.path.length);
    const baseline : string = version ?? 'current';
    const icon: string = 'gravity-ui:layout-header-cells-large-fill';
    addTab(name, icon, url, baseline);
}

export const load = async () => {
    return new Promise<void>((resolve, reject) => {
        reloadRepository()
        .then(() => {
            loadAuthorInformation();
            addTab('Home', 'gravity-ui:house', '/home');
            setActiveTab('/home');
            return resolve();
        })
    })
};
