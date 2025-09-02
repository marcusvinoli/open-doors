<script lang="ts">
    import Icon from "@iconify/svelte";
    import Button from "$lib/components/ui/button/button.svelte";
	import StringDropdown from "$lib/components/forms/module/StringDropdown.svelte";
	import StringComboBox from "$lib/components/forms/module/StringComboBox.svelte";
	
	import { Input } from "$lib/components/ui/input/index.js";
	import { Checkbox } from "$lib/components/ui/checkbox/index.js";

	import type { AttributeKind } from "$lib/components/structs/Attributes";

	let {
		attributeKind,
		value = $bindable(),
		kind = Object.keys(attributeKind.kind)[0],
		allowedValues = Object.values(attributeKind.kind)[0],
		disabled = true
	} : {
		attributeKind: AttributeKind;
		value: any;
		kind?: any;
		allowedValues?: any;
		disabled?: boolean;
	} = $props();

</script>

{#if kind === "enumeration"}
	<div class="flex flex-row gap-1">
		<StringDropdown options={allowedValues} bind:selected={value} placeholder="" disabled={disabled}/>
		{#if !disabled}
			<Button size="sm" variant="ghost" onclick={() => value = ""}>
				<Icon icon="gravity-ui:eraser" width="20px"/>
			</Button>
		{/if}
	</div>
{:else if kind === "optional"}
	<div class="flex flex-row gap-1">
		<StringComboBox options={allowedValues} bind:selectedList={value} placeholder="" disabled={disabled}/>
		{#if !disabled}
			<Button size="sm" variant="ghost" onclick={() => value = ""}>
				<Icon icon="gravity-ui:eraser" width="20px"/>
			</Button>
		{/if}
	</div>
{:else if kind === "boolean"}
	<div class="min-h-10 flex items-center">
		{#if value === "true"}
		<Checkbox class="mx-1" checked={true} onclick={() => (value = "false")} disabled={disabled}/>
		{:else}
		<Checkbox class="mx-1" checked={false} onclick={() => (value = "true")} disabled={disabled}/>
		{/if}
	</div>
{:else}
	<Input bind:value={value} disabled={disabled}/>
{/if}
