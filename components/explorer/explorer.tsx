import * as React from "react"
import { listFiles, File } from "@/lib/helsync"
import { FileEntry } from "./entry"

export type SortFileKey = 'name' | 'createdAt' | 'modifiedAt'

// Defines a common context for explorer.
export type ExplorerContextType = {

  // Control the available documents.
  documents: () => Array<FileEntry>
  setDocuments: (docs: Array<FileEntry>) => void

  // Control document sorting.
  sortFileKey: () => SortFileKey
  setSortFileKey: (key: SortFileKey) => void
  sortFileAsc: () => boolean
  setSortFileAsc: (asc: boolean) => void

  // Control explorer view.
  isViewDocuments: () => boolean
  setIsViewDocuments: (isViewDocuments: boolean) => void
}

// Implements ExporerContextType.
export const ExplorerContext = React.createContext<ExplorerContextType | null>(null)

// Exposes ExplorerContext.
export function ExplorerProvider({ children }: { children: React.ReactNode }) {
  const [documents, setDocuments] = React.useState<FileEntry[]>([])
  const [sortFileKey, setSortFileKey] = React.useState<SortFileKey>('name')
  const [sortFileAsc, setSortFileAsc] = React.useState<boolean>(true)
  const [isViewDocuments, setIsViewDocuments] = React.useState<boolean>(true)

  // Recursively builds a file tree.
  React.useEffect(() => {
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
    const fetchFiles = async () => {
      try {
        const tree = await buildTree()
        setDocuments(tree)
      } catch (error) {
        console.error("Failed to fetch file tree:", error)
      }
    }
    fetchFiles()
  }, [])

  const context: ExplorerContextType = {
    documents: () => { return documents },
    setDocuments: (docs) => { setDocuments(docs) },
    sortFileKey: () => { return sortFileKey },
    setSortFileKey: (key) => setSortFileKey(key),
    sortFileAsc: () => { return sortFileAsc },
    setSortFileAsc: (asc) => setSortFileAsc(asc),
    isViewDocuments: () => { return isViewDocuments },
    setIsViewDocuments: (isDocs) => setIsViewDocuments(isDocs)
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
