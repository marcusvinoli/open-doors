<script lang="ts">
	import Icon from "@iconify/svelte";
	import { cn } from "$lib/utils.js"; 
	import { tick } from "svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import { createEventDispatcher } from "svelte";
	import * as Command from "$lib/components/ui/command/index.js";
	import * as Popover from "$lib/components/ui/popover/index.js";

	export let options: string[] = [];
	export let selected: string = "";
	export let placeholder: string = "";
	export let disabled: boolean = false;

	const dispatch = createEventDispatcher();

	function handleSelection(select: string) {
		selected = select;
		dispatch('select', {item: selected})
	}

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
	
</script>

<div class="w-full">
	<Popover.Root bind:open={openCombobox} let:ids>
		<Popover.Trigger asChild let:builder>
		<Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full" disabled={disabled}>
			<p class="w-full text-left">{selected !== "" ? selected : placeholder}</p>
			<Icon icon="lucide:chevron-down" class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
		</Popover.Trigger>
		<Popover.Content class="p-0">
			<Command.Root class="w-full">
				<Command.Input placeholder="Search..."/>
				<Command.Empty><p class="text-left px-2 py-1">No match...</p></Command.Empty>
				<Command.Group class="max-h-32 overflow-scroll">
				{#each options as option}
					<Command.Item value={option} onSelect={() => { closeAndFocusTrigger(ids.trigger); handleSelection(option); }}>
						<Icon icon="lucide:check" class={cn("mr-2 h-4 w-4", !(option === selected) && "text-transparent" )}/>
						<span class="pl-1">{option}</span>
					</Command.Item>
				{/each}
				</Command.Group>
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
