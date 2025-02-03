<script lang="ts">
	import IndexTree from "$lib/components/global/indextree/IndexTree.svelte";
	import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
	import ObjectExplorer from "$lib/components/global/object_explorer/ObjectExplorer.svelte";
	import AttributesForm from "$lib/components/forms/module/AttributesForm.svelte";
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { addTab } from "$lib/stores/Tabs";
	import { onMount } from "svelte";
	import { pageState } from "../../store";
	import { repository } from "$lib/stores/Repository";
	import { defaultView } from "$lib/components/global/object_explorer/viewMethods";
	import { loadRepository } from "$lib/controllers/Repository";
	import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
	import { exportCSV, exportXlsx, readBaselinedObjects, readModuleFromPath } from "$lib/controllers/Module";
	import * as Resizable from "$lib/components/ui/resizable";
	import type { View } from "$lib/components/global/object_explorer/viewStructs";
	import type { Object } from "$lib/components/structs/Object";
	import type { Module } from "$lib/components/structs/Module";
	import type { IHash, Link, ObjectView } from "$lib/components/structs/Object";
	import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
	import type { Template } from "$lib/components/structs/Template";;
	
	let selectedObject: ObjectView | null = null;
	let objects: ObjectView[] = [];
	let module: Module;

	let templateFlag: boolean = false;
	let readOnlyFlag: boolean = true;
	let editPanelFlag: boolean = false;
	let treePanelFlag: boolean = false;
	let showLinksFlag: boolean = true;
	let showDeletionsFlag: boolean = false;
	let showRowNumberFlag: boolean = true;
	
	let tabKey: string = "";
	let view: View = defaultView();

	function loadHomeToolbar() {
		clearToolbar();
	
		let homeButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Home",
			icon: "gravity-ui:house",
			action: () => {
				goto("/home")
			},
		}

		let showTree: ToolbarButtonType = {
			type: "button",
			tooltip: "Show/Hide tree panel",
			icon: "gravity-ui:layout-header-side-content",
			action: () => {
				treePanelFlag = !treePanelFlag;
			},
		}

		let exportButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Export module...",
			icon: "gravity-ui:file-arrow-right-out",
			action: () => {},
		}

		let exportExcelButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Microsoft Excel (.xlsx)",
			icon: "ri:file-excel-2-fill",
			action: () => {
				exportXlsx(module.path).then((res) => console.log(res))
			},
		}

		let exportCSVButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Comma-Separeted Value (.csv)",
			icon: "ph:file-csv",
			action: () => {
				exportCSV(module.path).then((res) => console.log(res))
			},
		}

		let showDeletionsButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Show deletions",
			icon: "gravity-ui:square-chart-bar",
			action: () => {
				showDeletionsFlag = true;
			},
		}

		let dontShowDeletionsButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Don't show deletions",
			icon: "gravity-ui:square-dashed-text",
			action: () => {
				showDeletionsFlag = false;
			},
		}

		let deletionsModeButton: ToolbarToggleType = {
			type: "toggle",
			buttonOn: showDeletionsButton,
			buttonOff: dontShowDeletionsButton,
			status: showDeletionsFlag,
		}

		let expGroup: ToolbarDropdownType = {
			button: exportButton,
			items: [
				{
					items: [
						exportExcelButton,
					],
					type: "buttonsGroup",
				},
				{
					items: [
						exportCSVButton,
					],
					type: "buttonsGroup",
				},
			],
			type: "dropdown"
		}
	
		let navigationGroup: ToolbarGroupType = {
			items: [homeButton],
			type: "buttonsGroup"
		}
			
		let exportGroup: ToolbarGroupType = {
			items: [expGroup],
			type: "buttonsGroup"
		}

		let viewGrouplView: ToolbarGroupType = {
			items: [showTree, deletionsModeButton],
			type: "buttonsGroup"
		}
	
		addToolbarItem(navigationGroup);
		addToolbarItem(viewGrouplView);
		addToolbarItem(exportGroup);
	}
		
	function handleCloseEditPanel(event: any) {
		editPanelFlag = false;
	}

	function createCustomFieldHashFromTemplate(template: Template, customFields: IHash) {
		template.fields.forEach((field) => {
			if (!customFields[field.key]) {
				customFields[field.key] = "";
			}
		})
	}

	function handleObjectSelect(event: any) {
		if (event) { 
			selectedObject = event.detail.objectView; 
		}
		let customFields = selectedObject?.object.customFields || {};
		createCustomFieldHashFromTemplate(module.template, customFields)
		selectedObject!.object.customFields! = customFields;
		editPanelFlag = true;
		console.log("Open...")
	}

	function handleScrollIntoView(event: any) {
		scrollIntoView(event.detail.path);
	}
	
	function scrollIntoView(id: string) {
		const el = document.getElementById("row-" + id);
		const ov = document.getElementById("scroll-table");
		const hd = document.getElementById("scroll-table-header");
		if (!el) return;
		if (!ov) return;
		if (!hd) return;

		const offset = hd.offsetHeight;

		ov.scrollTo({
			top: el.offsetTop - offset,
			behavior: 'smooth'
		});

		document.querySelectorAll('.flash').forEach(element => {
			element.classList.remove('flash');
		});

		el.classList.add('flash');
	}

	function getScrollPosition() {
		const ov = document.getElementById("scroll-table");
		if (!ov) {return };
		return {x: ov.scrollTop, y: ov.scrollLeft};
	}
	
	function setScrollPosition(x: number, y: number) {
		const ov = document.getElementById("scroll-table");
		if (!ov) {return };
		ov.scrollTo({top: x, left: y, behavior: 'instant'})
	}

	function compareLevels(a: string, b: string): number {
		const parseLevel = (level: string) => level.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));
		
		const aParts = parseLevel(a);
		const bParts = parseLevel(b);
		
		const len = Math.max(aParts.length, bParts.length);
		for (let i = 0; i < len; i++) {
			if (aParts[i] === undefined) return -1;
			if (bParts[i] === undefined) return 1;
			
			if (typeof aParts[i] === 'number' && typeof bParts[i] === 'number') {
				if (aParts[i] !== bParts[i]) return (aParts[i] as number) - (bParts[i] as number);
			} else if (typeof aParts[i] === 'string' && typeof bParts[i] === 'string') {
				let aP = aParts[i] as string;
				let bP = bParts[i] as string;
				if (aParts[i] !== bParts[i]) return aP.localeCompare(bP);
			} else {
				return typeof aParts[i] === 'number' ? -1 : 1;
			}
		}
		return 0;
	}

	function getNewLevel(currentLevel: string, direction: 'sameLevel' | 'belowLevel'): string {
		const parts = currentLevel.split(/[\.\-]/).map(part => isNaN(Number(part)) ? part : Number(part));

		if (direction === 'belowLevel') {
			if (typeof parts[parts.length - 1] === 'number') {
			return currentLevel + '.1';
			} else {
			return currentLevel + '.1';
			}
		} else if (direction === 'sameLevel') {
			if (typeof parts[parts.length - 1] === 'number') {
			parts[parts.length - 1] = (parts[parts.length - 1] as number) + 1;
			} else {
			const lastPart = parts[parts.length - 1] as string;
			const newChar = String.fromCharCode(lastPart.charCodeAt(0) + 1);
			parts[parts.length - 1] = newChar;
			}
			return parts.join('.');
		}
		
		return currentLevel;
	} 

	function sortItems(items: ObjectView[]): ObjectView[] {
		return items.sort((a, b) => compareLevels(a.object.level, b.object.level));
	}

	function getLinks(links: any, id: number) {
		let ret: Link[] = [];

		if (links[id]) {
			links[id].forEach((lk: Link) => {
				ret.push(lk);
			})
		}

		return ret;
	}

	async function loadAllObjects(modPath: string, version: string) {
		let objs = await readBaselinedObjects(modPath, version) as Object[];
		objects = [];
		objs.forEach((obj) => {
			let dob: ObjectView = {
				object: obj as Object,
				isDraft: false,
				hasChanges: false,
				inboundLinks: getLinks(module.inboundLinks, obj.id),
			};
			objects.push(dob);
		})
	}

	async function loadModule(modPath: string) {
		// Maybe there must be a "readBaselinedModuleFromPath(modPath, version)..."
		module = await readModuleFromPath(modPath); 
	}

	async function load(modPath: string, version: string) {
		await loadModule(modPath);
		await loadAllObjects(modPath, version);
	}

	function generateKey(input: string): string {
		const sanitized = input.toLowerCase().replace(/[^a-z0-9]/g, '');
		const truncated = sanitized.length > 30 ? sanitized.substring(0, 30) : sanitized;
		return truncated;
	}

	function updateState(mod: string, version: string) {
		tabKey = generateKey(`${mod.substring($repository?.tree.path.length)}-${version}`);
		const savedState = pageState.getPageState(tabKey);
		if (savedState) {
			({ scrollX, scrollY, selectedObject, editPanelFlag, view, showLinksFlag, showRowNumberFlag, readOnlyFlag } = savedState);
			setScrollPosition(scrollX, scrollY);
		}
	}

	function saveCurrentState() {
		const scroll = getScrollPosition();
		const currentView = JSON.parse(JSON.stringify(view))
		const state = {
			scrollX: scroll?.x??0,
			scrollY: scroll?.y??0,
			view: currentView,
			showRowNumberFlag,
			selectedObject,
			editPanelFlag,
			showLinksFlag,
			readOnlyFlag,
		};
		pageState.setPageState(tabKey, state);
	}

	function setupPage() {
		const params = $page.params;
		const url: string = $page.url.pathname;
		const name: string = params.mod.substring($repository?.tree.path.length);
		const version: string = params.version;
		saveCurrentState();
		loadRepository();
		loadHomeToolbar();
		load(params.mod, version).then(() => {
			updateState(params.mod, params.version);
			const hash = $page.url.hash;
			if(hash && hash !== "") {
				scrollIntoView(hash.slice(1));
			}
		})
		addTab(name, "gravity-ui:layout-header-cells-large-fill", url, version);
	}
	
	$: {
		const { mod, version } = $page.params;
		setupPage();
	}
	
	onMount(async () => {
		//setupPage();
	})
	
</script>

<div class="bg-slate-50 h-full py-1">
	{#if module}
		<AttributesForm bind:module={module} bind:openDialog={templateFlag}/>
	{/if}
	<Resizable.PaneGroup direction="horizontal">
		{#if treePanelFlag}
			<Resizable.Pane defaultSize={20} collapsible order={1}>
				<IndexTree items={objects} on:click={handleScrollIntoView}/>
			</Resizable.Pane>
		<Resizable.Handle withHandle/>
		{/if}
			<Resizable.Pane order={2}>
				{#if module}
					<ObjectExplorer 
						readOnly={true} 
						bind:view={view} 
						bind:module={module} 
						bind:objects={objects} 
						bind:showLinks={showLinksFlag} 
						bind:showRowNumber={showRowNumberFlag} 
						bind:showDeleted={showDeletionsFlag}
						on:click={handleObjectSelect} 
						on:create={handleObjectSelect} 
					/>
				{/if}
			</Resizable.Pane>
		{#if editPanelFlag}
			<Resizable.Handle/>
			<Resizable.Pane class="h-full" defaultSize={50} order={3}>
				{#if selectedObject}
				<ObjectEditor 
				readOnlyMode={true}
				bind:objectView={selectedObject} 
				bind:module={module}
				on:close={handleCloseEditPanel}
				/>
				{/if}
			</Resizable.Pane>
		{/if}
	</Resizable.PaneGroup>
</div>
