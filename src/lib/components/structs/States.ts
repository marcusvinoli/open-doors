import type { Link } from "./Link";
import type { User } from "./User";
import type { Task } from "./Task";
import type { View } from "./View"
import type { Object } from "./Object";
import type { Module } from "./Module";
import type { Repository } from "./Repo";
import type { IndexItem } from "./IndexItem";

export interface IndexTreeState {
    tree: IndexItem[],
    state: Map<number, boolean>,
    scroll: {
        x: number,
        y: number,
    }
};

export interface Linker {
    from: Link | null,
    to: Link | null,
};

export interface ModuleFlags {
    showNewBaselineDialog: boolean;
    showTemplateDialog: boolean;
    showRowsNumbering: boolean;
    showObjectDialog: boolean;
    showIndexPanel: boolean;
    showDeletions: boolean;
    showLinks: boolean;
    readOnly: boolean;
};

export interface ModuleState {
    flags: ModuleFlags,
    module: Module;
    objects: Object[];
    currentView: View;
    currentObject: Object | null;
    indexTree: IndexTreeState;
    filter: string | null;
    scroll: {
        x: number,
        y: number,
    },
}

export interface AppState {
    user: User | null,
    repository: Repository | null,
    modules: Map<string, ModuleState>,
    tasks: Map<string, Task>,
    linker: Linker | null,
    currentModule: ModuleState | null,
}
