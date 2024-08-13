<script lang="ts">
	import Icon from "@iconify/svelte";
	import type { Object } from "$lib/components/structs/Object";

	export let key: string;
	export let object: Object;

</script>

<div>
	{#key object}
		{#if object.customFields}
			{#if object.customFields[key] === "true"}
				<div class="text-green-500 flex justify-center items-center">
					<Icon icon="gravity-ui:check" width="15px"/>
				</div>
			{:else if object.customFields[key] === "false"}
				<div class="text-red-500 flex justify-center items-center">
					<Icon icon="gravity-ui:xmark" width="15px"/>
				</div>
			{:else if object.customFields[key]}
				{object.customFields[key]}
			{:else}
				{""}
			{/if}
		{:else}
			{""}
		{/if}
	{/key}
</div>