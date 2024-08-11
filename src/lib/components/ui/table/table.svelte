<script lang="ts">
	import type { HTMLTableAttributes } from "svelte/elements";
	import { cn } from "$lib/utils.js";

	type $$Props = HTMLTableAttributes;

	let className: $$Props["class"] = undefined;
	let itemid: $$Props["itemid"] = undefined;
	export { className as class, itemid as id };
</script>

<div class="relative w-full overflow-auto" id={itemid}>
	<table class={cn("w-full caption-bottom text-sm", className)} {...$$restProps}>
		<slot />
	</table>
</div>
