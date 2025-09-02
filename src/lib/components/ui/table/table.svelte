<script lang="ts">
	import type { HTMLTableAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTableAttributes> = $props();
</script>

<div data-slot="table-container" class="relative h-full overflow-x-auto" id={restProps.id + "-container"}>
	<table
		bind:this={ref}
		data-slot="table"
		class={cn("w-full caption-bottom text-sm overflow-auto", className)}
		{...restProps}
	>
		{@render children?.()}
	</table>
</div>
