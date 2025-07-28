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

// Extend File to include children
export interface FileEntry extends File {
  children?: FileEntry[]
}

// Describes a change made to the filesystem. Sent as a tauri event.
export type FileChangeEvent = {
  event: "copy" | "move" | "remove" | "createFolder"
  data: object,
}

// Fetches metadata for the file with `id`.
export async function getFile(id: string): Promise<File> {
  return await invoke<File>("plugin:helsync|get_file", {
    payload: { id },
  })
}

// Copy a file.
//
// Copies a file and moves it under `parentId`. If `parentId` is
// unspecified, then the file is moved to the root directory. If
// `name` is specified, the file is renamed.
export async function copyFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|copy_file", {
    payload: { sourceId, parentId, name }
  })
}

// Move a file.
//
// Sets the file's parent to `parentId`. If `parentId` is
// unspecified, then the file is moved to the root directory. If
// `name` is specified, the file is renamed.
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

// Create a new directory.
//
// If `parentId` is unspecified, the directory is created at the
// filesystem root.
export async function createFolder(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_folder", {
    payload: { parentId, name }
  })
}

// Create a new file.
//
// If `parentId` is unspecified, the directory is created at the
// filesystem root.
export async function createFile(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_file", {
    payload: { parentId, name }
  })
}

// Lists all immediate files belonging to `parentId`.
//
// If `parentId` is unspecified, then it returns all files below
// the filesystem root.
export async function listFiles(parentId?: string): Promise<Array<File>> {
  return await invoke<Array<File>>("plugin:helsync|list_files", {
    payload: { parentId }
  })
}

// Write to a file in the filesystem.
export async function writeToFile(name: string, contents: Uint8Array, parentId?: string): Promise<File> {
    return await invoke<File>("plugin:helsync|write_to_file", {
        payload: {
            parentId,
            name,
            contents: Array.from(contents),
        }
    })
}
