<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
    import { readModuleFromPath } from '$lib/controllers/Module';
    import { encodePath } from '$lib/utils/pathHandler';
	
	let mod = '';

	$: {
		const { mod } = $page.params;
		const module = readModuleFromPath(mod);
	}

	onMount(async () => {
		try {
			const { mod } = $page.params;
			const hash = $page.url.hash;

			const module = await readModuleFromPath(mod);
			const baselines = module.baselines;

			if (baselines.length > 0) {
				baselines.sort((a: any, b: any) => semverCompare(b.version, a.version));
				const latestBaseline = baselines[0].version;
				goto("/module/" + encodePath(module.path) + "/baseline/" + latestBaseline + hash);
			} else {
				console.error('No baselines found for this module');
			}

		} catch (error) {
			console.error('Failed to fetch module data:', error);
		}
	});

	// Função para comparar versões semânticas
	function semverCompare(a: any, b: any) {
		const pa = a.split('.').map(Number);
		const pb = b.split('.').map(Number);
		for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
			const diff = (pa[i] || 0) - (pb[i] || 0);
			if (diff !== 0) {
				return diff;
			}
		}
		return 0;
	}
</script>
<div></div>