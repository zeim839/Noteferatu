import { invoke } from "@tauri-apps/api/core"

// A Helsync virtual file. Can be either a document or folder.
export type File = {
  id:         number,
  name:       string,
  parent?:    number,
  remoteId?:  string,
  isDeleted:  boolean,
  createdAt:  number,
  modifiedAt: number,
  syncedAt?:  number,
  isFolder:   boolean,
}

// Describes a change made to the filesystem. Sent as a tauri event.
export type FileChangeEvent = {
  event: "copy" | "move" | "remove" | "createFolder"
  data: object,
}

// Reads the file with the given `id`.
export async function getFile(id: string): Promise<File> {
  return await invoke<File>("plugin:helsync|get_file", {
    payload: { id },
  })
}

// Copy the file with `sourceId` to the parent with `parentId`,
// optionally renaming it to `name`.
export async function copyFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|copy_file", {
    payload: { sourceId, parentId, name }
  })
}

// Move the file with `sourceId` to the parent with `parentId`,
// optionally renaming it to `name`.
export async function moveFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|move_file", {
    payload: { sourceId, parentId, name }
  })
}

// Delete the file with the given `id`.
export async function removeFile(id: string): Promise<null> {
  return await invoke("plugin:helsync|remove_file", {
    payload: { id }
  })
}

// Create a new directory with name `name` at the parent
// `parentId`. If `parentId` is unspecified, the directory is
// created at the filesystem root.
export async function createFolder(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_folder", {
    payload: { parentId, name }
  })
}

// Lists all immediate files belonging to `parent_id`. If
// `parent_id` is unspecified, then it returns all files below
// the filesystem root.
export async function listFiles(parentId?: string): Promise<Array<File>> {
  return await invoke<Array<File>>("plugin:helsync|list_files", {
    payload: { parentId }
  })
}

// Write to a file in the filesystem, creating it if it doesn't
// exist.
export async function writeToFile(name: string, contents: Uint8Array, parentId?: string): Promise<File> {
    return await invoke<File>("plugin:helsync|write_to_file", {
        payload: {
            parentId,
            name,
            contents: Array.from(contents),
        }
    })
}
