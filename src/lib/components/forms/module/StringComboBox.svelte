<script lang="ts">
	import Check from "lucide-svelte/icons/check";
	import ChevronDown from "lucide-svelte/icons/chevron-down";
	import { cn } from "$lib/utils.js"; 
	import { onMount, tick } from "svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import * as Command from "$lib/components/ui/command/index.js";
	import * as Popover from "$lib/components/ui/popover/index.js";

	export let options: string[] = [];
	export let selected: string[] = [];
	export let placeholder: string = "";
	export let selectedList: string = placeholder;
	export let disabled: boolean = false;

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

	function changeSelection(item: string) {
		let index = selected.findIndex((opt) => (opt === item))
		if (index < 0) {
			selected.push(item);
		} else {
			selected.splice(index, 1);
		}
		generateList();
	}

	function generateList() {
		if (selected.length > 0) {
			selectedList = selected.join(", ");
		} else {
			selectedList = placeholder;
		}
	}

	onMount(() => {
		if (selectedList) {
			selected = selectedList.split(",").map(s => s.trim());
		} else if (selected) {
			generateList();
		}
	})

</script>

<Popover.Root bind:open={openCombobox} let:ids closeFocus>
	<Popover.Trigger asChild let:builder class="w-full">
	<Button builders={[builder]} variant="outline" role="combobox" aria-expanded={openCombobox} class="justify-between w-full" disabled={disabled} >
		<p class="w-full text-left">{(placeholder !== "") ? placeholder : selectedList}</p>
		<ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
	</Button>
	</Popover.Trigger>
	<Popover.Content class="p-0">
	<Command.Root>
		<Command.Group class="">
			{#each options as option}
			<Command.Item value={option} onSelect={() => { changeSelection(option); closeAndFocusTrigger(ids.trigger); }}>
				<Check class={cn("mr-2 h-4 w-4", !(selected.includes(option)) && "text-transparent" )}/> 
				<span class="pl-1">{option}</span>
			</Command.Item>
			{/each}
		</Command.Group>
	</Command.Root>
	</Popover.Content>
</Popover.Root>
