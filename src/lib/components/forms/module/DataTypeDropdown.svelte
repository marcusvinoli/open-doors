<script lang="ts">
	import Icon from '@iconify/svelte';
	import Check from "lucide-svelte/icons/check";
	import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
	import { cn } from "$lib/utils.js";
	import { tick } from "svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import { onMount } from 'svelte';
	import * as Command from "$lib/components/ui/command/index.js";
	import * as Popover from "$lib/components/ui/popover/index.js";

	let dataTypes: any[] = [
		{
			name: "Text",
			dataType: "any",
			icon: "gravity-ui:text"
		},
		{
			name: "Single Option",
			dataType: "nullableOption",
			icon: "gravity-ui:circle-check"
		},
		{
			name: "Multiple Options",
			dataType: "nullableOptions",
			icon: "gravity-ui:square-check"
		},
		{
			name: "Yes/No",
			dataType: "nullableBoolean",
			icon: "gravity-ui:copy-check-xmark"
		},
	];

	let selection: string;
	export let dataType: string;

	let openCombobox: boolean = false;
	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger(triggerId: string) {
		openCombobox = false;
		tick().then(() => {
		document.getElementById(triggerId)?.focus();
		});
	}

	function handleClick(item: string) {
		selection = item;
		let seleIndex = dataTypes.findIndex(item => {return (item.name === selection)})
		dataType = dataTypes[seleIndex].dataType;
	}

	onMount(() => {
		if(dataType) {
			let seleIndex = dataTypes.findIndex(item => {return (item.dataType === dataType)})
			if(seleIndex < 0) { seleIndex = 0; }
			selection = dataTypes[seleIndex].name;
		} else {
			selection = dataTypes[0].name;
			handleClick(selection);
		}
	})
</script>

<Popover.Root bind:open={openCombobox} let:ids >
	<Popover.Trigger asChild let:builder>
		<Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full">
			{selection}
			<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="p-0 w-38">
		<Command.Root>
			<Command.Group class="overflow-scroll">
				{#each dataTypes as dt}
				<Command.Item value={dt.name} onSelect={() => { handleClick(dt.name); closeAndFocusTrigger(ids.trigger); }}>
					<Check class={cn("mr-2 h-4 w-4", selection !== dt.name && "text-transparent" )}/> 
					<Icon icon={dt.icon} width="15px"/>
					<span class="pl-2">{dt.name}</span>
				</Command.Item>
				{/each}
			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
