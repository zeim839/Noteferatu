import * as React from "react"
import { listen } from '@tauri-apps/api/event'

import {
  listFiles,
  listBookmarks,
  listTags,
  File,
  FileEntry,
  TagWithFiles,
} from "@/lib/helsync"

export type SortFileKey = 'name' | 'createdAt' | 'modifiedAt'
export type ViewType = 'documents' | 'tags' | 'bookmarks'

// Defines a common context for explorer.
export type ExplorerContextType = {

  // Control the available documents.
  documents: () => Array<FileEntry>
  setDocuments: (docs: Array<FileEntry>) => void

  // Control bookmarks.
  bookmarks: () => Array<FileEntry>

  // Control tags.
  tags: () => Array<TagWithFiles>

  // Control document sorting.
  sortFileKey: () => SortFileKey
  setSortFileKey: (key: SortFileKey) => void
  sortFileAsc: () => boolean
  setSortFileAsc: (asc: boolean) => void

  // Control explorer view.
  view: () => ViewType
  setView: (view: ViewType) => void
}

// Implements ExporerContextType.
export const ExplorerContext = React.createContext<ExplorerContextType | null>(null)

// Exposes ExplorerContext.
export function ExplorerProvider({ children }: { children: React.ReactNode }) {
  const [documents, setDocuments] = React.useState<FileEntry[]>([])
  const [bookmarks, setBookmarks] = React.useState<FileEntry[]>([])
  const [tags, setTags] = React.useState<TagWithFiles[]>([])
  const [sortFileKey, setSortFileKey] = React.useState<SortFileKey>('name')
  const [sortFileAsc, setSortFileAsc] = React.useState<boolean>(true)
  const [view, setView] = React.useState<ViewType>('documents')

  // Recursively fetch files, directories, and their children.
  const buildTree = async (parentId?: string): Promise<FileEntry[]> => {
    const files = await listFiles(parentId)
    return Promise.all(
      files.map(async (file: File): Promise<FileEntry> => {
        const entry: FileEntry = { ...file }
        if (file.isFolder) {
          entry.children = await buildTree(file.id.toString())
        }
        return entry
      }),
    )
  }

  // Fetch files from helsync.
  const fetchFiles = async () => {
    try {
      const tree = await buildTree()
      setDocuments(tree)
    } catch (error) {
      console.log(error)
    }
  }

  // Fetch bookmarks from helsync.
  const fetchBookmarks = async () => {
    try {
      const bookmarkedFiles = await listBookmarks()
      const bookmarkedEntries = await Promise.all(
        bookmarkedFiles.map(async (file: File): Promise<FileEntry> => {
          const entry: FileEntry = { ...file }
          if (file.isFolder) {
            entry.children = await buildTree(file.id.toString())
          }
          return entry
        }),
      )
      setBookmarks(bookmarkedEntries)
    } catch (error) {
      console.log(error)
    }
  }

  // Fetch tags from helsync and build their file trees.
  const fetchTags = async () => {
    try {
      const allTags = await listTags()
      const tagsWithPopulatedFiles = await Promise.all(
        allTags.map(async (tag: TagWithFiles): Promise<TagWithFiles> => {
          const populatedFiles = await Promise.all(
            tag.files.map(async (file: FileEntry): Promise<FileEntry> => {
              if (file.isFolder) {
                file.children = await buildTree(file.id.toString())
              }
              return file
            }),
          )
          return { ...tag, files: populatedFiles }
        }),
      )
      setTags(tagsWithPopulatedFiles)
    } catch (error) {
      console.log(error)
    }
  }

  // Initial file fetch and register event listener.
  React.useEffect(() => {
    fetchFiles()
    fetchBookmarks()
    fetchTags()
    const fsEventPromise = listen("helsync-fs-change", () => {
      fetchBookmarks()
      fetchFiles()
      fetchTags()
    })
    const bookmarkEventPromise = listen("helsync-bookmark-change", () => {
      fetchBookmarks()
    })
    const tagsEventPromise = listen("helsync-tags-change", () => {
      fetchTags()
    })
    return () => {
      fsEventPromise.then((unlisten) => unlisten())
      bookmarkEventPromise.then((unlisten) => unlisten())
      tagsEventPromise.then((unlisten) => unlisten())
    }
  }, [])

  // Construct ExplorerContextType.
  const context: ExplorerContextType = {
    documents: () => { return documents },
    setDocuments: (docs) => { setDocuments(docs) },
    bookmarks: () => { return bookmarks },
    tags: () => { return tags },
    sortFileKey: () => { return sortFileKey },
    setSortFileKey: (key) => setSortFileKey(key),
    sortFileAsc: () => { return sortFileAsc },
    setSortFileAsc: (asc) => setSortFileAsc(asc),
    view: () => { return view },
    setView: (view) => setView(view),
  }

  return (
    <ExplorerContext.Provider value={context}>
      {children}
    </ExplorerContext.Provider>
  )
}

export function useExplorerContext() {
  const context = React.useContext(ExplorerContext)
  if (!context) {
    throw new Error("useExplorerContext must be called within ExplorerProvider")
  }
  return context
}
