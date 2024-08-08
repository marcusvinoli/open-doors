import type { View } from '$lib/components/global/object_explorer/viewStructs';
import type { ObjectView } from '$lib/components/structs/Object';
import { writable } from 'svelte/store';

export interface PageState {
  scrollX: number;
  scrollY: number;
  selectedObject: ObjectView | null;
  editPanelFlag: boolean;
  view: View;
}

const createPageState = () => {
  const { subscribe, set, update } = writable<{ [key: string]: PageState }>({});

  const setPageState = (tabKey: string, state: PageState) => {
    update(states => {
      states[tabKey] = state;
      return states;
    });
  };

  const getPageState = (tabKey: string) => {
    let state: PageState | undefined;
    subscribe(states => {
      state = states[tabKey];
    })();
    return state;
  };

  return {
    subscribe,
    setPageState,
    getPageState,
  };
};

export const pageState = createPageState();