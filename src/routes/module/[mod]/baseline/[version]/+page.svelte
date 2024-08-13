<script lang="ts">
	import IndexTree from "$lib/components/global/indextree/IndexTree.svelte";
	import ObjectEditor from "$lib/components/global/object_editor/ObjectEditor.svelte";
	import ObjectExplorer from "$lib/components/global/object_explorer/ObjectExplorer.svelte";
	import AttributesForm from "$lib/components/forms/module/AttributesForm.svelte";
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
    import { user } from "$lib/stores/User";
	import { addTab } from "$lib/stores/Tabs";
	import { confirm } from '@tauri-apps/api/dialog';
	import { pageState } from "./store";
	import { repository } from "$lib/stores/Repository";
	import { defaultView } from "$lib/components/global/object_explorer/viewMethods";
	import { loadRepository } from "$lib/controllers/Repository";
	import { beforeUpdate, onMount } from "svelte";
	import { addToolbarItem, clearToolbar } from "$lib/stores/Toolbar";
	import { createDraftObject, createObject, deleteObject, readDraftObjects, readModuleFromPath, readObjects } from "$lib/controllers/Module";
	import * as Resizable from "$lib/components/ui/resizable";
	import type { View } from "$lib/components/global/object_explorer/viewStructs";
	import type { Module } from "$lib/components/structs/Module";
	import type { IHash, Link, ObjectView } from "$lib/components/structs/Object";
	import type { ToolbarButtonType, ToolbarDropdownType, ToolbarGroupType, ToolbarToggleType } from "$lib/components/global/toolbar/Toolbar";
    import type { Template } from "$lib/components/structs/Template";
	
	let selectedObject: ObjectView | null = null;
	let objects: ObjectView[] = [];
	let module: Module;

	let templateFlag: boolean = false;
	let editModeFlag: boolean = true;
	let editPanelFlag: boolean = false;
	let treePanelFlag: boolean = false;
	let showLinksFlag: boolean = true;
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
			action: () => {},
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

		let readOnlyModeButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Toggle Edit Mode",
			icon: "lucide:pencil-off",
			action: () => {
				editModeFlag = false;
			},
		}

		let editModeButton: ToolbarButtonType = {
			type: "button",
			tooltip: "Toggle Read-Only Mode",
			icon: "lucide:pencil",
			action: () => {
				editModeFlag = true;
			},
		}

		let viewModeButton: ToolbarToggleType = {
			type: "toggle",
			buttonOn: editModeButton,
			buttonOff: readOnlyModeButton,
			status: editModeFlag,
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
	
		let navigationGroup: ToolbarGroupType = {
			items: [homeButton],
			type: "buttonsGroup"
		}
	
		let newGroup: ToolbarGroupType = {
			items: [creationGroup],
			type: "buttonsGroup"
		}

		let viewGrouplView: ToolbarGroupType = {
			items: [showTree, viewModeButton],
			type: "buttonsGroup"
		}

		let templateButton: ToolbarGroupType = {
			items: [templateManager],
			type: "buttonsGroup"
		}
	
		addToolbarItem(navigationGroup);
		addToolbarItem(newGroup);
		addToolbarItem(viewGrouplView);
		addToolbarItem(templateButton);

	}

	function createCustomFieldHashFromTemplate(template: Template, customFields: IHash) {
        template.fields.forEach((field) => {
            if (!customFields[field.key]) {
                customFields[field.key] = "";
            }
        })
    }

	function createEmptyObject(): ObjectView {
        let customFields: IHash = {};
        createCustomFieldHashFromTemplate(module.template, customFields);
        return {
            object: {
                id: 0,
                header: "",
                content: "",
                author: $user.toString()!,
                isActive: true,
                isNormative: false,
                isRequirement: false,
                createdAt: new Date(),
                updatedAt: new Date(),
                deletedAt: null,
                customFields: customFields,
                level: "",
                outboundLinks: [],
            },
            inboundLinks: [],
            isDraft: false,
            hasChanges: false,
        }
    }

	function handleObjectCreation(event: any) {
		let obj = event.detail.objectView.object;
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
		let obj = event.detail.objectView.object;
		console.log("Saving...", obj)
		createDraftObject(module.path, obj)
			.then((objs) => {
				console.log("Saved...", objs)
				selectedObject = createEmptyObject();
				editPanelFlag = false;
				loadAllObjects(module.path);
			})
			.catch((err) => {
				console.log(err);
			})
	}
	
	async function handleObjectExclusion(event: any) {
		let obj = event.detail.objectView.object;
		const confirmed = await confirm('Do you really want to delete this Object?', 'Deleting object ' + module.manifest.prefix + module.manifest.separator + obj.id );
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
	
	function handleCloseEditPanel(event: any) {
		editPanelFlag = false;
	}

	function handleObjectSelect(event: any) {
		if (event) { 
			selectedObject = event.detail.objectView; 
		}
		let customFields = selectedObject?.object.customFields || {};
		createCustomFieldHashFromTemplate(module.template, customFields)
		selectedObject!.object.customFields! = customFields;
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
		let currObj = event.detail.objectView as ObjectView;
		let currentLevel = currObj.object.level;
		selectedObject = createEmptyObject();
		selectedObject.object.level = getNewLevel(currentLevel, 'sameLevel');
		handleObjectSelect(undefined);
	}

	function handleCreateObjectNextLevel(event: any) {
		let currObj = event.detail.objectView as ObjectView;
		let currentLevel = currObj.object.level;
		selectedObject = createEmptyObject();
		selectedObject.object.level = getNewLevel(currentLevel, 'belowLevel');
		handleObjectSelect(undefined);
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

	async function loadAllObjects(modPath: string) {
		let retObjects = await readObjects(modPath);
		let retDraftObjects = await readDraftObjects(modPath);
		let newObjects: ObjectView[] = [];
		console.log("Objects", retObjects);
		console.log("Draft Objects", retDraftObjects);

		retObjects.forEach((obj) => {
			let dob = {
				object: obj,
				isDraft: false,
				hasChanges: false,
				inboundLinks: getLinks(module.inboundLinks, obj.id),
			}
			newObjects.push(dob);
		});

		retDraftObjects.forEach((dobj) => {
			let index = newObjects.findIndex((ob) => {return (ob.object.id === dobj.id)});
			let dob = {
				object: dobj,
				isDraft: true,
				hasChanges: false,
				inboundLinks: getLinks(module.inboundLinks, dobj.id),
			}

			if (index < 0) {
				newObjects.push(dob);
			} else {
				newObjects[index] = dob;
			}
		});
		newObjects = sortItems(newObjects);
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
			({ scrollX, scrollY, selectedObject, editPanelFlag, view, showLinksFlag, showRowNumberFlag } = savedState);
			setScrollPosition(scrollX, scrollY);
		}
	}

	function setupPage() {
		const params = $page.params;
		const url: string = $page.url.pathname;
		const name: string = params.mod.substring($repository?.tree.path.length);
		const version: string = params.version;
		loadRepository();
		loadHomeToolbar();
		addTab(name, "gravity-ui:layout-header-cells-large-fill", url, version);
		load(params.mod).then(() => {
			updateState(params.mod, params.version);
			const hash = $page.url.hash;
			if(hash && hash !== "") {
				scrollIntoView(hash.slice(1));
			}
		})
	}
	
	$: {
		const { mod, version } = $page.params;
		setupPage();
	}
	
	onMount(async () => {
		setupPage();
	})
	
	beforeUpdate(() => {
		const scroll = getScrollPosition();
		const state = {
			scrollX: scroll?.x??0,
			scrollY: scroll?.y??0,
			selectedObject,
			editPanelFlag,
			view,
			showLinksFlag,
			showRowNumberFlag,
		};
		pageState.setPageState(tabKey, state);
	});

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
						bind:view={view} 
						bind:module={module} 
						bind:objects={objects} 
						bind:editMode={editModeFlag} 
						bind:showLinks={showLinksFlag} 
						bind:showRowNumber={showRowNumberFlag} 
						on:click={handleObjectSelect} 
						on:create={handleObjectSelect}
						on:commit={handleObjectCreation} 
						on:delete={handleObjectExclusion} 
						on:createBelow={handleCreateObjectBelow} 
					/>
				{/if}
			</Resizable.Pane>
		{#if editPanelFlag && editModeFlag}
			<Resizable.Handle/>
			<Resizable.Pane class="h-full" defaultSize={50} order={3}>
				{#if selectedObject}
				<ObjectEditor 
				bind:objectView={selectedObject} 
				bind:module={module} 
				on:save={handleObjectCreation} 
				on:close={handleCloseEditPanel} 
				on:delete={handleObjectExclusion}
				on:saveDraft={handleObjectDraftCreation} 
				/>
				{/if}
			</Resizable.Pane>
		{/if}
	</Resizable.PaneGroup>
</div>