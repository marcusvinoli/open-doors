<script lang="ts">
    import Icon from "@iconify/svelte";
    import Button from "$lib/components/ui/button/button.svelte";
	import StringDropdown from "$lib/components/forms/module/StringDropdown.svelte";
	import StringComboBox from "$lib/components/forms/module/StringComboBox.svelte";
	import { Input } from "$lib/components/ui/input/index.js";
	import { Checkbox } from "$lib/components/ui/checkbox/index.js";
	import type { Field } from "$lib/components/structs/Template";
	
	export let value: any;
	export let field: Field;
	export let type = Object.keys(field.kind)[0];
	export let allowedValues = Object.values(field.kind)[0];
	export let disabled: boolean = true;

</script>

{#if type === "nullableOption"}
	<div class="flex flex-row gap-1">
		<StringDropdown options={allowedValues} bind:selected={value} placeholder="" disabled={disabled}/>
		{#if !disabled}
			<Button size="sm" variant="ghost" on:click={() => value = ""}>
				<Icon icon="gravity-ui:eraser" width="20px"/>
			</Button>
		{/if}
	</div>
{:else if type === "nullableOptions"}
	<div class="flex flex-row gap-1">
		<StringComboBox options={allowedValues} bind:selectedList={value} placeholder="" disabled={disabled}/>
		{#if !disabled}
			<Button size="sm" variant="ghost" on:click={() => value = ""}>
				<Icon icon="gravity-ui:eraser" width="20px"/>
			</Button>
		{/if}
	</div>
{:else if type === "nullableBoolean"}
	<div class="min-h-10 flex items-center">
		{#if value === "true"}
		<Checkbox class="mx-1" checked={true} on:click={() => (value = "false")} disabled={disabled}/>
		{:else}
		<Checkbox class="mx-1" checked={false} on:click={() => (value = "true")} disabled={disabled}/>
		{/if}
	</div>
{:else}
	<Input bind:value={value} disabled={disabled}/>
{/if}
