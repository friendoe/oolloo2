import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export const preventDefault = <T extends Event>(fn: (e: T) => void): ((e: T) => void) => {
    return (e: T) => {
        e.preventDefault();
        fn(e);
    };
};

export enum FILES {
    GREET_FILE = 'greet.txt',
    NAME_FILE = 'name.txt'
}

export class GlobalState {
    private _state = $state({ name: '', greet: '' });

    get greet() {
        return this._state.greet;
    }

    set greet(value: string) {
        this._state.greet = value;
    }

    get name() {
        return this._state.name;
    }

    set name(value: string) {
        this._state.name = value;
    }

    get nlen() {
        return this.name.length;
    }

    get glen() {
        return this.greet.length;
    }

    async read(path: FILES) {
        const contentFromFile = await invoke<string>('read', { path });
        if (path === FILES.NAME_FILE) {
            this.name = contentFromFile;
        } else if (path === FILES.GREET_FILE) {
            this.greet = contentFromFile;
        }
    }

    async write(path: FILES, contents: string) {
        await invoke('write', { path, contents });
        if (path === FILES.NAME_FILE) {
            this.name = contents;
        } else if (path === FILES.GREET_FILE) {
            this.greet = contents;
        }
    }

    reset() {
        this.name = '';
        this.greet = '';
    }
}

export const commands = {
    fetchAllModels: async (query: string | null) => await invoke<any[]>('fetch_all_models', { query }),
    fetchModelDetails: async (url: string) => await invoke<any[]>('fetch_model_details', { url }),
    pullModel: async (model: string) => await invoke('pull_model', { model }),
    listen: listen,
};
