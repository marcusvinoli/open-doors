<script lang="ts">
	import Icon from "@iconify/svelte";
	import Input from "$lib/components/ui/input/input.svelte";
	import DataTypeDropdown from "./DataTypeDropdown.svelte";
	import { Button } from "$lib/components/ui/button/index.js";
	import { createEventDispatcher } from 'svelte';
	import { readModuleFromPath, saveTemplate } from "$lib/controllers/Module";
	import * as Dialog from "$lib/components/ui/dialog/index.js";
	import * as Table from "$lib/components/ui/table";
	import type { Module } from "$lib/components/structs/Module";
	import type { Field, DataKind } from "$lib/components/structs/Template";

	const dispatch = createEventDispatcher();

	export let openDialog: boolean = false;
	export let module: Module;
	let tempModule: Module;

	let newAttribute: string;
	let newDataTypeSelected: string;
	let newAllowedValues: string;
	
	function closeDialog() {
		openDialog = false;
	}

	function generateKey(input: string): string {
		const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
		const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
		return truncated;
	}

	function generateDataKind(dataType: string, allowedValues: string = ""): DataKind {
		switch (dataType) {
			case "nullableOptions":
				return { nullableOptions: allowedValues.split(",").map(s => s.trim()) };
			case "nullableOption": 
				return { nullableOption: allowedValues.split(",").map(s => s.trim()) };
			case "nullableBoolean":
				return { nullableBoolean: false };
			default:
				return { any: "" };
		}
	}

	function addAttribute() {
		if (newAttribute === "") {
			return;
		}

		let newField: Field = {
			attribute: newAttribute,
			kind: generateDataKind(newDataTypeSelected, newAllowedValues),
			key: generateKey(newAttribute),
		}

		tempModule.template.fields = [...tempModule.template.fields, newField];

		newDataTypeSelected = "any";
		newAllowedValues = "";
		newAttribute = "";
	}

	function removeAttribute(key: string) {
		let tempTemplte = tempModule.template.fields;
		let index = tempTemplte.findIndex((fd) => (fd.key === key));
		if (index < 0) {
			return;
		}
		tempTemplte.splice(index, 1);
		tempModule.template.fields = tempTemplte;
	}

	function getDataKind(data: any) {
		let dataKind = Object.keys(data)[0];
		switch (dataKind) {
			case "any": 
				return "Text";
			case "nullableOption":
				return "Single Option";
			case "nullableOptions":
				return "Multiple Options";
			case "nullableBoolean":
				return "Yes/No";
		}
	}

	function getDataValues(data: any) {
		let dataKind = Object.keys(data)[0];
		switch (dataKind) {
			case "nullableOption":
			case "nullableOptions":
				let dataValues = Object.values(data)[0] as string[];
				return dataValues.join(", ");
			case "nullableBoolean": 
				return "Boolean values";
			case "any": 
				return "String values";
			default: 
				return "";
		}
	}

	async function handleSaveTemplate() {
		await saveTemplate(module.path, tempModule.template);
		readModuleFromPath(module.path)
			.then(mod => {
				module = mod as Module;
				dispatch('updateTemplate', {})
			})
			.finally(() => {
				closeDialog();
			})
	}

	$: if (openDialog && module) {
		tempModule = JSON.parse(JSON.stringify(module));
	}
</script>

{#if tempModule}
<Dialog.Root bind:open={openDialog} closeOnEscape closeOnOutsideClick>
	<Dialog.Content class="max-w-[75%] max-h-[80%] flex flex-col">
		<Dialog.Header class="pt-2 pb-1">
			<Dialog.Title>Custom Attributes of {tempModule.manifest.prefix} Formal Module</Dialog.Title>
			<Dialog.Description>{tempModule.manifest.title} module</Dialog.Description>
		</Dialog.Header>
		<Table.Root class="w-full max-h-32 overflow-auto">
			<Table.Header class="w-full">
				<Table.Row class="border-b-[1px]">
					<Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Attribute</Table.Head>
					<Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[200px]">Data Type</Table.Head>
					<Table.Head class="sticky top-0 bg-slate-50 shadow-sm">Values</Table.Head>
					<Table.Head class="sticky top-0 bg-slate-50 shadow-sm w-[30px]"></Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body class="w-full">
				<Table.Row class="">
					<Table.Cell class="pl-2 pr-1">
						<Input bind:value={newAttribute} placeholder="New attribute..." class="px-2 py-1 w-full"/>
					</Table.Cell>
					<Table.Cell class="px-1">
						<DataTypeDropdown bind:dataType={newDataTypeSelected} />
					</Table.Cell>
					<Table.Cell class="px-1">
						<Input bind:value={newAllowedValues} placeholder="Comma, Separeted, Values" class="px-2 py-1 w-full" disabled={(newDataTypeSelected === "any") || (newDataTypeSelected === "nullableBoolean")}/>
					</Table.Cell>
					<Table.Cell class="w-[30px] pl-1 pr-2">
						<Button variant="ghost" class="hover:text-blue-600" on:click={addAttribute} disabled={(newAttribute === "")}>
							<Icon icon="gravity-ui:circle-plus" width="20px" />
						</Button>
					</Table.Cell>
				</Table.Row>
				{#each tempModule.template.fields as field}
				<Table.Row>
					<Table.Cell>
						<input bind:value={field.attribute} placeholder="Create a new attribute..." class="bg-transparent px-2 py-1 w-full"/>
					</Table.Cell>
					<Table.Cell>
						{getDataKind(field.kind)}
					</Table.Cell>
					<Table.Cell>
						{getDataValues(field.kind)}
					</Table.Cell>
					<Table.Cell class="w-[30px] pl-1 pr-2">
						<Button variant="ghost" class="hover:text-red-600" on:click={() => removeAttribute(field.key)}>
							<Icon icon="gravity-ui:circle-minus" width="20px" />
						</Button>
					</Table.Cell>
				</Table.Row>
				{:else}
				<Table.Row class="h-28">
				</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
		<Dialog.Footer>
			<div class="grow"></div>
			<Button variant="secondary" on:click={closeDialog}>Cancel</Button>
			<Button on:click={handleSaveTemplate}>Save Changes</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
{/if}
