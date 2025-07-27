"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { listFiles, File } from "@/lib/helsync"
import { FileEntry, Entry } from "./entry"

import {
  FolderIcon,
  ChevronDownIcon,
  ArrowDownWideNarrowIcon,
  SlidersHorizontalIcon,
} from "lucide-react"

// File explorer shows a tree of all available directories, documents,
// and their subchildren.
function Explorer() {
  const [documents, setDocuments] = React.useState<FileEntry[]>([])
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())
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

  return (
    <div className="w-full max-h-[calc(100vh-35px)] min-w-[200px] flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 min-h-[29px]">
        <div className="flex flex-row items-center gap-1">
          <Button variant="ghost" size="icon">
            <FolderIcon strokeWidth={1.6} />
          </Button>
          <p className="text-xs">Documents</p>
          <Button variant="ghost" size="icon">
            <ChevronDownIcon strokeWidth={1.6} />
          </Button>
        </div>
        <div className="flex flex-row">
          <Button variant="ghost" size="icon" tooltip="Filter / Sort">
            <ArrowDownWideNarrowIcon strokeWidth={1.6} />
          </Button>
          <Button variant="ghost" size="icon" tooltip="Customize View">
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>
      <div className="w-full flex flex-col px-1 pt-1 flex-1 overflow-auto scrollbar-hide relative">
        {documents.map((doc, i) => (
          <Entry
            key={doc.id}
            file={doc}
            expandedFolders={expandedFolders}
            setExpandedFolders={setExpandedFolders}
            isLast={i === documents.length - 1}
          />
        ))}
      </div>
    </div>
  )
}

export { Explorer }
