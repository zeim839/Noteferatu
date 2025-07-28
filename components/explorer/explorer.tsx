"use client"

import * as React from "react"
import { useExplorerContext } from "./context"
import { ExplorerHeader } from "./header"
import { ExplorerContextMenu } from "./menus/explorer"
import { Entry } from "./entry"
import { FileEntry } from "@/lib/helsync"

// File explorer shows a tree of all available directories, documents,
// and their subchildren.
function Explorer() {
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())
  const explorer = useExplorerContext()

  // Compares two file entries (used for sorting).
  const compareFn = (a: FileEntry, b: FileEntry): number => {
    const [keyA, keyB] = [a[explorer.sortFileKey()], b[explorer.sortFileKey()]]
    const asc = explorer.sortFileAsc()
    if (keyA < keyB) {
      return asc ? -1 : 1
    }
    if (keyA > keyB) {
      return asc ? 1 : -1
    }
    return 0
  }

  return (
    <ExplorerContextMenu className="w-full max-h-[calc(100vh-35px)] h-[calc(100vh-35px)] min-w-[200px] flex flex-col">
      <ExplorerHeader />
      <div className="w-full flex flex-col px-1 pt-1 flex-1 overflow-auto scrollbar-hide relative">
        {
          (explorer.isViewDocuments()) ?
            [...explorer.documents()].sort(compareFn).map((doc, i) => (
              <Entry
                key={doc.id}
                file={doc}
                expandedFolders={expandedFolders}
                setExpandedFolders={setExpandedFolders}
                isLast={i === explorer.documents.length - 1}
                sortFileKey={explorer.sortFileKey}
                sortFileAsc={explorer.sortFileAsc}
              />
            )) : null
        }
      </div>
    </ExplorerContextMenu>
  )
}

export { Explorer }
