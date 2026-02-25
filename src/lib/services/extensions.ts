import { open } from '@tauri-apps/plugin-dialog';
import { readFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { basename } from '@tauri-apps/api/path'; // To get the filename from a path

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
        // Open file selection dialog
        const selectedPath = await open({
            multiple: false,
            filters: [{
                name: 'Executable Files',
                extensions: ['exe', 'bin', 'sh', 'app'] // Adjust as needed for common binary types
            }, {
                name: 'All Files',
                extensions: ['*']
            }]
        });

        if (!selectedPath) {
            throw new Error("File selection cancelled.");
        }

        // Extract the filename from the path
        const fileName = await basename(selectedPath);
        if (!fileName) {
            throw new Error("Could not determine filename from selected path.");
        }

        // Read the binary content of the file
        const fileContent = await readFile(selectedPath);

        // Invoke the Rust backend command
        try {
            await invoke('upload_extension', { 
                name: fileName, 
                data: Array.from(fileContent) // Convert Uint8Array to Rust's Vec<u8>
            });
            console.log(`Extension '${fileName}' uploaded successfully.`);
            return fileName; // Return the name of the uploaded file
        } catch (error) {
            console.error(`Failed to upload extension '${fileName}':`, error);
            throw new Error(`Failed to upload extension: ${error}`);
        }
    },

    // Other extension-related methods will go here (list, run, delete, etc.)
};
