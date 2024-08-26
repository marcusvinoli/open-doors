<script lang="ts">
	import Icon from "@iconify/svelte";
    import CustomFieldCell from "./CustomFieldCell.svelte";
	import { Button } from "$lib/components/ui/button";
	import { marked } from "marked";
	import { defaultView, parseTemplate } from "./viewMethods";
	import { createEventDispatcher, onMount } from "svelte";
	import * as Table from "$lib/components/ui/table";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
	import type { View } from "./viewStructs";
	import type { Module } from "$lib/components/structs/Module";
	import type { ObjectView, Object } from "$lib/components/structs/Object";
	import "./markdown.css";
	import "./flashing.css";

	export let objects: ObjectView[] = [];
	export let module: Module;
	export let view: View;

	export let readOnly: boolean = true;
	export let showLinks: boolean = true;
	export let showDeleted: boolean = false;
	export let showRowNumber: boolean = true;

	let objs: ObjectView[] = [];

	const dispatch = createEventDispatcher();

	function onEditClick(obj: ObjectView) {
		dispatch("select", {objectView: obj})
	}

	function handleRowClick(obj: ObjectView) {
		dispatch('click', {objectView: obj})
	}

	function handleCommit(obj: ObjectView) {
		dispatch('commit', {objectView: obj})
	}

	function handleDelete(obj: ObjectView) {
		dispatch('delete', {objectView: obj})
	}

	function handleCreate(obj: ObjectView) {
		dispatch('create', {objectView: obj})
	}

	function handleCreateBelow(obj: ObjectView) {
		dispatch('createBelow', {objectView: obj})
	}

	function generateHashString(level: string): string {
		const delimiters = /[\.\-a-zA-Z]/;
		const parts = level.split(delimiters);
		const numericParts = parts.filter(part => part.match(/^\d+$/));
		const hashCount = Math.min(numericParts.length, 6);
		return '#'.repeat(hashCount) + " ";
	}

	function toggleRowNumberView() {
		showRowNumber = !showRowNumber;
	}

	function toggleLinksView() {
		showLinks = !showLinks;
	}
	
	$: {
		objs = objects;
		if(view.items.length > 0) {
			parseTemplate(module.template).forEach((item) => {
				if (view.items.findIndex((tv) => tv.key === item.key) < 0) {
					view.items.push(item);
				}
			})
		}
	}

	let tableHeaderClass: string = ""
	let tableCellClass: string = "text-sm "

	onMount(() => {
		if (view.items.length === 0 || !view) {
			view = defaultView();
		}
	})

</script>

<div class="h-full w-full flex flex-col">
	{#if objects.length > 0 && view}
		<Table.Root class="w-full relative" id="scroll-table">
			<Table.Header class="w-full min-w-96" id="scroll-table-header">
				<Table.Row class="">
					{#if showRowNumber}
					<Table.Head class="sticky top-0 bg-slate-50 shadow-sm">
						<ContextMenu.Root>
							<ContextMenu.Trigger>
								<div class={tableHeaderClass}>
									#
								</div>
							</ContextMenu.Trigger>
							<ContextMenu.Content>
							<ContextMenu.Item on:click={toggleRowNumberView}>
								{(showRowNumber)? "Hide" : "Show"} Row Number
								</ContextMenu.Item>
								<ContextMenu.Item on:click={toggleLinksView}>
								{(showLinks)? "Hide" : "Show"} Object Links
								</ContextMenu.Item>
							</ContextMenu.Content>
						</ContextMenu.Root>
					</Table.Head>
					{/if}
					{#each view.items as attributes}
						{#if attributes.show}
						<Table.Head class="sticky top-0 bg-slate-50 shadow-sm">
							<ContextMenu.Root>
								<ContextMenu.Trigger>
									<div class={tableHeaderClass}>
										{attributes.attribute}
									</div>
								</ContextMenu.Trigger>
								<ContextMenu.Content>
									<ContextMenu.Sub>
										<ContextMenu.SubTrigger>View</ContextMenu.SubTrigger>
										<ContextMenu.SubContent class="w-48">
											{#each view.items as vw}
											<ContextMenu.CheckboxItem bind:checked={vw.show}>
												{vw.attribute}
											</ContextMenu.CheckboxItem>
											{/each}
										</ContextMenu.SubContent>
									</ContextMenu.Sub>
									<ContextMenu.Separator />
									<ContextMenu.Item on:click={toggleRowNumberView}>
									{(showRowNumber)? "Hide" : "Show"} Row Number
									</ContextMenu.Item>
									<ContextMenu.Item on:click={toggleLinksView}>
									{(showLinks)? "Hide" : "Show"} Object Links
									</ContextMenu.Item>
								</ContextMenu.Content>
							</ContextMenu.Root>
						</Table.Head>
						{/if}
					{/each}
					{#if !readOnly}
					<Table.Head class="min-w-[50px] sticky top-0 bg-slate-50 shadow-sm">
					</Table.Head>
					{/if}
				</Table.Row>
			</Table.Header>
			<Table.Body class="w-full min-w-96">
				{#each objs as ov, index}
				{#if !(ov.object.deletedAt && showDeleted)}
					<Table.Row class={(ov.object.deletedAt ? "bg-rose-50 hover:bg-red-200" : "")} id={"row-" + ov.object.id.toString()} on:click={() => {handleRowClick(ov)}}>
						{#if showRowNumber}
							<Table.Cell class={tableCellClass}>{index + 1}</Table.Cell>
						{/if}
						{#each view.items as attr}
							{#if attr.show}
								{#if attr.key === "id"}
									<Table.Cell class={tableCellClass}>{module.manifest.prefix}{module.manifest.separator}{ov.object.id}</Table.Cell>
								{:else if attr.key === "content"}
									<Table.Cell class={tableCellClass + (ov.isDraft ? " border-l-2 border-l-yellow-500" : "")}>
									{#if ov.object.deletedAt}
										<div class="flex flex-row">
											<div class="markdown min-w-[320px] grow">
												<s>
													{#if ov.object.header !== ""}
													{@html marked(generateHashString(ov.object.level) + ov.object.level + " " + ov.object.header)}
													{/if}
													{@html marked(ov.object.content)}
												</s>
											</div>
											{#if showLinks}
												<div class="flex flex-col justify-center gap-2 pl-2">
													<div class="text-red-500 p1">
														{#if ov.object.outboundLinks?.length??0 > 0}
														<Tooltip.Root openDelay={200}>
															<Tooltip.Trigger>
																<Icon icon="lucide:arrow-up-right" width="15px" stroke=3/>
															</Tooltip.Trigger>
															<Tooltip.Content>
																<p>{ov.object.outboundLinks?.length??0} outbound {ov.object.outboundLinks?.length === 1 ? "link" : "links"}</p>
															</Tooltip.Content>
														</Tooltip.Root>
														{/if}
													</div>
													<div class="text-yellow-500 p1">
														{#if ov.inboundLinks.length > 0}
														<Tooltip.Root openDelay={200}>
															<Tooltip.Trigger>
																<Icon icon="lucide:arrow-down-left" width="15px" stroke-width=5/>
															</Tooltip.Trigger>
															<Tooltip.Content>
																<p>{ov.inboundLinks.length} inbound {ov.inboundLinks.length === 1 ? "link" : "links"}</p>
															</Tooltip.Content>
														</Tooltip.Root>
														{/if}
													</div>
												</div>
											{/if}
										</div>
									{:else}
										<ContextMenu.Root closeOnOutsideClick closeFocus closeOnEscape>
											<ContextMenu.Trigger class="w-full">
											<div class="flex flex-row items-center">
												<div class="markdown min-w-[320px] grow">
													{#if ov.object.header !== ""}
													{@html marked(generateHashString(ov.object.level) + ov.object.level + " " + ov.object.header)}
													{/if}
													{@html marked(ov.object.content)}
												</div>
												{#if showLinks}
													<div class="flex flex-col justify-center gap-2 pl-2">
														<div class="text-red-500 p1">
															{#if ov.object.outboundLinks?.length??0 > 0}
															<Tooltip.Root openDelay={200}>
																<Tooltip.Trigger>
																	<Icon icon="lucide:arrow-up-right" width="15px" stroke=3/>
																</Tooltip.Trigger>
																<Tooltip.Content>
																	<p>{ov.object.outboundLinks?.length??0} outbound {ov.object.outboundLinks?.length === 1 ? "link" : "links"}</p>
																</Tooltip.Content>
															</Tooltip.Root>
															{/if}
														</div>
														<div class="text-yellow-500 p1">
															{#if ov.inboundLinks.length > 0}
															<Tooltip.Root openDelay={200}>
																<Tooltip.Trigger>
																	<Icon icon="lucide:arrow-down-left" width="15px" stroke=3/>
																</Tooltip.Trigger>
																<Tooltip.Content>
																	<p>{ov.inboundLinks.length} inbound {ov.inboundLinks.length === 1 ? "link" : "links"}</p>
																</Tooltip.Content>
															</Tooltip.Root>
															{/if}
														</div>
													</div>
												{/if}
											</div>
											</ContextMenu.Trigger>
											<ContextMenu.Content>
												{#if ov.isDraft}
												<ContextMenu.Item on:click={() => {handleCommit(ov)}}>Commit changes</ContextMenu.Item>
												{/if}
												<ContextMenu.Item on:click={() => {handleDelete(ov)}}>Delete</ContextMenu.Item>
												<ContextMenu.Separator />
												<ContextMenu.Sub>
													<ContextMenu.SubTrigger class="w-48">New...</ContextMenu.SubTrigger>
													<ContextMenu.SubContent>
														<ContextMenu.Item on:click={() => {handleCreate(ov)}}>Object</ContextMenu.Item>
														<ContextMenu.Item on:click={() => {handleCreateBelow(ov)}}>Object below</ContextMenu.Item>
													</ContextMenu.SubContent>
												</ContextMenu.Sub>
											</ContextMenu.Content>
										</ContextMenu.Root>	
									{/if}	
									</Table.Cell>
								{:else if attr.key === "isActive"}
									<Table.Cell class="font-medium w-[20px]">
										{#if ov.object.isActive}
										<div class="text-green-500 flex justify-center items-center">
											<Icon icon="gravity-ui:check" width="15px"/>
										</div>
										{:else}
										<div class="text-red-500 flex justify-center items-center">
											<Icon icon="gravity-ui:xmark" width="15px"/>
										</div>
										{/if}
									</Table.Cell>
								{:else if attr.key === "isNormative"}
									<Table.Cell class="font-medium w-[20px]">
										{#if ov.object.isNormative}
										<div class="text-green-500 flex justify-center items-center">
											<Icon icon="gravity-ui:check" width="15px"/>
										</div>
										{:else}
										<div class="text-red-500 flex justify-center items-center">
											<Icon icon="gravity-ui:xmark" width="15px"/>
										</div>
										{/if}
									</Table.Cell>
								{:else if attr.key === "isRequirement"}
									<Table.Cell class="font-medium w-[20px]">
										{#if ov.object.isRequirement}
										<div class="text-green-500 flex justify-center items-center">
										<Icon icon="gravity-ui:check" width="15px"/>
										</div>
										{:else}
										<div class="text-red-500 flex justify-center items-center">
											<Icon icon="gravity-ui:xmark" width="15px"/>
										</div>
										{/if}
									</Table.Cell>
								{:else if attr.key === "author"}
									<Table.Cell class={tableCellClass}>
										<div class="text-sm italic text-gray-800">
											{ov.object.author}
										</div>
									</Table.Cell>
								{:else}
									<Table.Cell class={tableCellClass}>
										<CustomFieldCell object={ov.object} key={attr.key} />
									</Table.Cell>
								{/if}
							{/if}
						{/each}
						{#if !readOnly}
						<Table.Head class="w-[30px] text-center">
							<Table.Cell class="w-[30px]">
								{#if !ov.object.deletedAt}
								<Button variant="ghost" on:click={() => onEditClick(ov)}>
									<div class="flex justify-center items-center">
										<Icon icon="gravity-ui:pencil-to-square" width="20px"/>
									</div>
								</Button>
								{/if}
							</Table.Cell>
						</Table.Head>
						{/if}
					</Table.Row>				
				{/if}
				{/each}
			</Table.Body>
		</Table.Root>
	{:else}
		<div class="w-full h-full grow flex flex-col items-center justify-center text-slate-400 pb-[100px] rounded-lg">
			<Icon icon="gravity-ui:layout-header-cells-large-fill" width="50px"/>
			<h1 class="text-xl font-semibold my-1">EMPTY MODULE</h1>
		</div>
	{/if}
</div>
