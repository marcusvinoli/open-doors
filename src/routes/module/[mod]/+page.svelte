<script lang="ts">
	import IndexTree from "$lib/components/global/indextree/IndexTree.svelte";
	import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
	import ObjectExplorer from "$lib/components/global/object_explorer/ObjectExplorer.svelte";
	import AttributesForm from "$lib/components/forms/module/AttributesForm.svelte";
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { user } from "$lib/stores/User";
	import { addTab } from "$lib/stores/Tabs";
	import { onMount } from "svelte";
	import { confirm } from '@tauri-apps/api/dialog';
	import { pageState } from "./store";
	import { repository } from "$lib/stores/Repository";
	import { defaultView } from "$lib/controllers/View";
	import { loadRepository } from "$lib/controllers/Repository";
	import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
	import { createDraftObject, createObject, deleteObject, exportCSV, exportXlsx, readDraftObjects, readModuleFromPath, readObjects, restoreObject } from "$lib/controllers/Module";
	import * as Resizable from "$lib/components/ui/resizable";
	import type { View } from "$lib/components/structs/View";
	import type { Module } from "$lib/components/structs/Module";
	import type { IHash, Object } from "$lib/components/structs/Object";
	import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
	import type { Template } from "$lib/components/structs/Template";
    import BaselineForm from "$lib/components/forms/module/BaselineForm.svelte";
	import ToolbarButton from "$lib/components/global/toolbar/ToolbarButton.svelte";
	import ToolbarDropdown from "$lib/components/global/toolbar/ToolbarDropdown.svelte";
	import ToolbarGroup from "$lib/components/global/toolbar/ToolbarGroup.svelte";
    import { ObjectStatus } from "$lib/components/structs/ObjectStatus";
    import DynamicTable from "$lib/components/global/object_explorer/DynamicTable.svelte";
	
	let selectedObject: Object | null = null;
	let objects: Object[] = [];
	let module: Module;

	let templateFlag: boolean = false;
	let readOnlyFlag: boolean = false;
	let editPanelFlag: boolean = false;
	let treePanelFlag: boolean = false;
	let showLinksFlag: boolean = true;
	let showDeletionsFlag: boolean = false;
	let showRowNumberFlag: boolean = true;
	let newBaselineFlag: boolean = false;
	
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

		let templateManager: ToolbarButtonType = {
			type: "button",
			tooltip: "Custom Attributes",
			icon: "gravity-ui:shapes-3",
			action: () => {
				templateFlag = !templateFlag;
			}
		}
		
		let newButton: ToolbarButtonType = {
			type: "button",
			tooltip: "New...",
			icon: "gravity-ui:circle-plus",
			action: () => {},
		}
	
		let newBaselineButton: ToolbarButtonType = {
			type: "button",
			tooltip: "New Baseline",
			icon: "gravity-ui:tag",
			action: () => {
				newBaselineFlag = !newBaselineFlag;
			},
		}
	
		let newObjectButton: ToolbarButtonType = {
			type: "button",
			tooltip: "New Object",
			icon: "gravity-ui:square-chart-bar",
			action: () => {
				if(!editPanelFlag) {
					selectedObject = createEmptyObject();
					editPanelFlag = true;
				}
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

		let readOnlyModeButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Toggle Edit Mode",
			icon: "lucide:pencil-off",
			action: () => {
				readOnlyFlag = true;
			},
		}

		let editModeButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Toggle Read-Only Mode",
			icon: "lucide:pencil",
			action: () => {
				readOnlyFlag = false;
			},
		}

		let viewModeButton: ToolbarToggleType = {
			type: "toggle",
			buttonOn: editModeButton,
			buttonOff: readOnlyModeButton,
			status: readOnlyFlag,
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
		
		let creationGroup: ToolbarDropdownType = {
			button: newButton,
			items: [
				{
					items: [
						newObjectButton,
					],
					type: "buttonsGroup",
				},
				{
					items: [
						newBaselineButton,
					],
					type: "buttonsGroup",
				}
			],
			type: "dropdown",
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
	
		let newGroup: ToolbarGroupType = {
			items: [creationGroup],
			type: "buttonsGroup"
		}
		
		let exportGroup: ToolbarGroupType = {
			items: [expGroup],
			type: "buttonsGroup"
		}

		let viewGrouplView: ToolbarGroupType = {
			items: [showTree, viewModeButton, deletionsModeButton],
			type: "buttonsGroup"
		}

		let templateButton: ToolbarGroupType = {
			items: [templateManager],
			type: "buttonsGroup"
		}
	
		addToolbarItem(navigationGroup);
		addToolbarItem(newGroup);
		addToolbarItem(viewGrouplView);
		addToolbarItem(exportGroup);
		addToolbarItem(templateButton);

	}

	function createCustomFieldHashFromTemplate(template: Template, customFields: IHash) {
		template.fields.forEach((field) => {
			if (!customFields[field.key]) {
				customFields[field.key] = "";
			}
		})
	}

	function createEmptyObject(): Object {
		let customFields: IHash = {};
		createCustomFieldHashFromTemplate(module.template, customFields);
		let object: Object = {
            id: 0,
            parentLevel: 0,
            indexLevel: 0,
            header: "",
            content: "",
            author: $user.toString(),
            createdAt: new Date(),
            updatedAt: new Date(),
            deletedAt: null,
            attributes: null,
            metadata: {
                status: ObjectStatus.draft,
                inboundLinks: undefined,
                outboundLinks: undefined
            },
        }
		return object
	}

	function handleObjectCreation(event: any) {
		let obj = event.detail.object;
		createObject(module.path, obj)
			.then(() => {
				selectedObject = createEmptyObject();
				editPanelFlag = false;
				loadAllObjects(module.path);
			})
			.catch((err) => {
				console.log(err);
			})
	}
	
	function handleObjectDraftCreation(event: any) {
		let obj = event.detail.object;
		createDraftObject(module.path, obj)
			.then((objs) => {
				selectedObject = createEmptyObject();
				editPanelFlag = false;
				loadAllObjects(module.path);
			})
			.catch((err) => {
				console.log(err);
			})
	}
	
	async function handleObjectExclusion(event: any) {
		let obj = event.detail.object;
		const confirmed = await confirm('Do you really want to delete this Object?', 'Deleting object ' + module.manifest.prefix + module.manifest.separator + obj.id);
		if (!confirmed) {
			return;
		}
		deleteObject(module.path, obj.id)
			.then(() => {
				editPanelFlag = false;
				selectedObject = createEmptyObject();
				loadAllObjects(module.path);
			})
			.catch((err) => {
				console.log(err);
			})
	}

	async function handleObjectRestoring(event: any) {
		let obj = event.detail.object;
		const confirmed = await confirm('Do you really want to restore this Object?', 'Restoring object ' + module.manifest.prefix + module.manifest.separator + obj.id);
		if (!confirmed) {
			return;
		}
		restoreObject(module.path, obj.id)
			.then(() => {
				editPanelFlag = true;
				loadAllObjects(module.path);
			})
			.catch((err) => {
				console.error(err);
			})
	}
	
	function handleCloseEditPanel(event: any) {
		editPanelFlag = false;
	}

	function handleObjectSelect(event: any) {
		if (event) { 
			selectedObject = event.detail.object; 
		}
		let customFields = selectedObject?.attributes || {};
		createCustomFieldHashFromTemplate(module.template, customFields)
		selectedObject!.attributes! = customFields;
		editPanelFlag = true;
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

	function handleCreateObjectBelow(event: any) {
		let currentObject = event.detail.object;
		selectedObject = createEmptyObject();
		selectedObject.parentLevel = currentObject.id;
		selectedObject.indexLevel = 0;
		handleObjectSelect(undefined);
	}

	function handleCreateObjectNextLevel(event: any) {
		let currentObject = event.detail.object;
		selectedObject = createEmptyObject();
		selectedObject.parentLevel = currentObject.id;
		selectedObject.indexLevel = 1;
		handleObjectSelect(undefined);
	}

	/* function sortItems(items: Object[]): Object[] {
		return items.sort((a, b) => compareLevels(a.object.level, b.object.level));
	} */

	async function loadAllObjects(modPath: string) {
		let retObjects = await readObjects(modPath);
		let retDraftObjects = await readDraftObjects(modPath);
		let newObjects: Object[] = [];

		retObjects.forEach((obj) => {
			newObjects.push(obj);
		});

		retDraftObjects.forEach((dobj) => {
			let index = newObjects.findIndex((ob) => {return (ob.id === dobj.id)});
			if (index < 0) {
				newObjects.push(dobj);
			} else {
				newObjects[index] = dobj;
			}
		});
		//newObjects = sortItems(newObjects);
		objects = newObjects;
	}

	async function loadModule(modPath: string) {
		module = await readModuleFromPath(modPath);
	}

	async function load(modPath: string) {
		await loadModule(modPath);
		await loadAllObjects(modPath);
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
		const version: string = "current";
		saveCurrentState();
		loadRepository();
		loadHomeToolbar();
		load(params.mod).then(() => {
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
</script>

<div class="bg-slate-50 h-full py-1">
	{#if module}
		<BaselineForm bind:openDialog={newBaselineFlag} modulePath={module.path}/>
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
<!-- 					<ObjectExplorer 
						bind:view={view} 
						bind:module={module} 
						bind:objects={objects} 
						bind:readOnly={readOnlyFlag} 
						bind:showLinks={showLinksFlag} 
						bind:showRowNumber={showRowNumberFlag} 
						bind:showDeleted={showDeletionsFlag}
						on:click={handleObjectSelect} 
						on:create={handleObjectSelect}
						on:commit={handleObjectCreation} 
						on:delete={handleObjectExclusion} 
						on:createBelow={handleCreateObjectBelow} 
					/> -->
					<DynamicTable moduleManifest={module.manifest} objects={objects}/>
				{/if}
			</Resizable.Pane>
		{#if editPanelFlag}
			<Resizable.Handle/>
			<Resizable.Pane class="h-full" defaultSize={50} order={3}>
				{#if selectedObject}
				<ObjectEditor 
				bind:object={selectedObject} 
				bind:module={module} 
				bind:readOnlyMode={readOnlyFlag}
				on:save={handleObjectCreation} 
				on:close={handleCloseEditPanel} 
				on:delete={handleObjectExclusion}
				on:retore={handleObjectRestoring}
				on:saveDraft={handleObjectDraftCreation} 
				/>
				{/if}
			</Resizable.Pane>
		{/if}
	</Resizable.PaneGroup>
</div>
