<script lang="ts">
	import Icon from "@iconify/svelte";
	import TableCell from "$lib/components/ui/table/table-cell.svelte";
	import type { Object } from "$lib/components/structs/Object";

	export let key: string;
	export let object: Object;
	export let cellClass: string = "";
</script>

<TableCell class={cellClass}>
	{#if (object.customFields[key])}
		{#if object.customFields[key] === "true"}
			<div class="text-green-500 flex justify-center items-center">
				<Icon icon="gravity-ui:check" width="15px"/>
			</div>
		{:else if object.customFields[key] === "false"}
			<div class="text-red-500 flex justify-center items-center">
				<Icon icon="gravity-ui:xmark" width="15px"/>
			</div>
		{:else}
			{object.customFields[key]}
		{/if}
	{:else}
		{""}
	{/if}
</TableCell>