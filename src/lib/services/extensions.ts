import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { basename } from '@tauri-apps/api/path';
import { platform } from '@tauri-apps/plugin-os';

export interface Extension {
    id: string;
    name: string;
    version: string;
    isRunning: boolean;
}

export const ExtensionService = {
    /**
     * Opens a file dialog, reads the selected binary file, and uploads it as an extension.
     * The chosen filename is used directly, so ensure it follows the naming convention
     * "Base Name - Version.ext" (e.g., "My Extension - v1.0.0.bin").
     *
     * @returns {Promise<string>} The filename of the uploaded extension on success.
     * @throws {Error} If file selection is cancelled, read fails, or upload fails.
     */
    async add(): Promise<string> {
        await invoke('set_dialog_status', { isOpen: true });

        try {
            const currentPlatform = platform();
            let extensions = ['bin', 'sh'];

            if (currentPlatform === 'windows') {
                extensions = ['exe', 'bat', 'cmd'];
            } else if (currentPlatform === 'macos') {
                extensions.push('app');
            }

            const selectedPath = await open({
                multiple: false,
                filters: [{
                    name: 'Executable Files',
                    extensions: extensions
                }, {
                    name: 'All Files',
                    extensions: ['*']
                }]
            });

            if (!selectedPath) {
                throw new Error("File selection cancelled.");
            }

            await invoke('upload_extension', { 
                sourcePath: selectedPath
            });

            const fileName = await basename(selectedPath);
            return fileName || "unknown extension";
        } finally {
            await invoke('set_dialog_status', { isOpen: false });
        }
    },

    /**
     * Fetches the list of installed extensions from the backend.
     * @returns {Promise<Extension[]>} A sorted list of extension info.
     */
    async list(): Promise<Extension[]> {
        try {
            return await invoke<Extension[]>('list_extensions');
        } catch (error) {
            console.error("Failed to list extensions:", error);
            throw new Error(`Failed to list extensions: ${error}`);
        }
    },

    /**
     * Executes the extension binary.
     * @param id The full filename of the extension.
     */
    async run(id: string): Promise<void> {
        try {
            await invoke('run_extension', { id });
        } catch (error) {
            console.error(`Failed to run extension ${id}:`, error);
            throw new Error(`${error}`);
        }
    },

    /**
     * Terminates the extension binary.
     * @param id The full filename of the extension.
     */
    async stop(id: string): Promise<void> {
        try {
            await invoke('stop_extension', { id });
        } catch (error) {
            console.error(`Failed to stop extension ${id}:`, error);
            throw new Error(`${error}`);
        }
    },

    /**
     * Deletes the extension binary and stops it if running.
     * @param id The full filename of the extension.
     */
    async delete(id: string): Promise<void> {
        try {
            await invoke('delete_extension', { id });
        } catch (error) {
            console.error(`Failed to delete extension ${id}:`, error);
            throw new Error(`${error}`);
        }
    },

    // Other extension-related methods will go here
};
