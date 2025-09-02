import { writable } from 'svelte/store';

import type { Object } from '$lib/components/structs/Object';
import type { View } from '$lib/components/structs/View';

export interface PageState {
  scrollX: number;
  scrollY: number;
  selectedObject: Object | null;
  editPanelFlag: boolean;
  showLinksFlag: boolean,
	showRowNumberFlag: boolean,
  readOnlyFlag: boolean,
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