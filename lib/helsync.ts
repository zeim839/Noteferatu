import { invoke } from "@tauri-apps/api/core"
import { Node } from "./markdown"

// A Helsync virtual file. Can be either a document or folder.
export type File = {
  id:           number
  name:         string
  parent?:      number
  remoteId?:    string
  isDeleted:    boolean
  createdAt:    number
  modifiedAt:   number
  syncedAt?:    number
  isFolder:     boolean
  isBookmarked: boolean
}

// A tag organizes file entries under a common category (name).
export type Tag = {
  name:      string
  color:     string
  createdAt: number
}

export interface HelsyncError {
  type: "helsync"
  error: {
    type: string
    error: string
  }
}

// Extend File to include children
export interface FileEntry extends File {
  children?: FileEntry[]
}

// A tag with all of its member files.
export type TagWithFiles = {
  name: string
  color: string
  files: Array<FileEntry>
}

// Fetches metadata for the file with `id`.
export async function getFile(id: string): Promise<File> {
  return await invoke<File>("plugin:helsync|get_file", {id})
}

// Copy a file.
//
// Copies a file and moves it under `parentId`. If `parentId` is
// unspecified, then the file is moved to the root directory. If
// `name` is specified, the file is renamed.
export async function copyFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|copy_file", {
    sourceId, parentId, name
  })
}

// Move a file.
//
// Sets the file's parent to `parentId`. If `parentId` is
// unspecified, then the file is moved to the root directory. If
// `name` is specified, the file is renamed.
export async function moveFile(sourceId: string, parentId?: string, name?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|move_file", {
    sourceId, parentId, name
  })
}

// Delete the file with the given `id`.
export async function removeFile(id: string): Promise<void> {
  return await invoke("plugin:helsync|remove_file", {id})
}

// Create a new directory.
//
// If `parentId` is unspecified, the directory is created at the
// filesystem root.
export async function createFolder(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_folder", {
    parentId, name
  })
}

// Create a new file.
//
// If `parentId` is unspecified, the directory is created at the
// filesystem root.
export async function createFile(name: string, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|create_file", {
    parentId, name
  })
}

// Lists all immediate files belonging to `parentId`.
//
// If `parentId` is unspecified, then it returns all files below
// the filesystem root.
export async function listFiles(parentId?: string): Promise<Array<File>> {
  return await invoke<Array<File>>("plugin:helsync|list_files", {
    parentId
  })
}

// Write to a file in the filesystem.
export async function writeToFile(name: string, contents: Uint8Array, parentId?: string): Promise<File> {
  return await invoke<File>("plugin:helsync|write_to_file", {
    parentId, name, contents: Array.from(contents),
  })
}

// Read from a file in the filesystem.
export async function readFromFile(id: string): Promise<Node> {
  return await invoke<Node>("plugin:helsync|read_from_file", {id})
}

// Fetch all bookmarked files.
export async function listBookmarks(): Promise<Array<File>> {
  return await invoke<Array<File>>("plugin:helsync|list_bookmarks")
}

// Bookmark a file.
export async function createBookmark(file_id: string): Promise<void> {
  return await invoke("plugin:helsync|create_bookmark", {id: file_id})
}

// Remove a bookmark from a file..
export async function removeBookmark(file_id: string): Promise<void> {
  return await invoke("plugin:helsync|remove_bookmark", {id: file_id})
}

// List all available tags, including those with no associated files.
export async function listTags(): Promise<Array<TagWithFiles>> {
  return await invoke<Array<TagWithFiles>>("plugin:helsync|list_tags", {})
}

// Create a new tag.
export async function createTag(name: string, color: string): Promise<Tag> {
  return await invoke<Tag>("plugin:helsync|create_tag", {name, color})
}

// Remove a tag and all its tag binds.
export async function removeTag(name: string): Promise<void> {
  return await invoke("plugin:helsync|remove_tag", {name})
}

// Change a tag's color.
export async function changeTagColor(name: string, color: string): Promise<void> {
  return await invoke("plugin:helsync|change_tag_color", {
    name, color
  })
}

// Attach a tag to a file.
export async function createTagBind(fileId: string, tagName: string): Promise<void> {
  return await invoke("plugin:helsync|create_tag_bind", {
    fileId, tagName
  })
}

// Remove a tag from a file.
export async function removeTagBind(fileId: string, tagName: string): Promise<void> {
  return await invoke("plugin:helsync|remove_tag_bind", {
    fileId, tagName
  })
}
