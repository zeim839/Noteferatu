import { invoke } from "@tauri-apps/api/core"

type File = {
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

export async function getFile(id: string): Promise<File> {
  return await invoke<File>("plugin:helsync|get_file", {
    payload: { id },
  })
}

export async function copyFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|copy_file", {
    payload: { sourceId, parentId, name }
  })
}

export async function moveFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|move_file", {
    payload: { sourceId, parentId, name }
  })
}

export async function removeFile(id: string): Promise<null> {
  return await invoke("plugin:helsync|remove_file", {
    payload: { id }
  })
}

export async function createFolder(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_folder", {
    payload: { parentId, name }
  })
}

export async function listFiles(parentId?: string): Promise<Array<File>> {
  return await invoke<Array<File>>("plugin:helsync|list_files", {
    payload: { parentId }
  })
}
