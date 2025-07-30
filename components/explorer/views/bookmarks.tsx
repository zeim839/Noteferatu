import * as React from "react"
import { useExplorerContext } from "../context"
import { FileEntry } from "@/lib/helsync"
import { Entry } from "../entry"

// Displays a list of bookmarks sorted in a particular order when the
// explorer view is set to 'bookmarks'.
function BookmarksView() {
  const explorer = useExplorerContext()
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())

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
    <div
      data-is-view-bookmarks={explorer.view() === "bookmarks"}
      className="w-full flex flex-col px-1 pt-1 flex-1 overflow-auto scrollbar-hide relative data-[is-view-bookmarks=false]:hidden"
    >
      {
        [...explorer.bookmarks()].sort(compareFn).map((doc, i) => (
          <Entry
            key={doc.id}
            file={doc}
            expandedFolders={expandedFolders}
            setExpandedFolders={setExpandedFolders}
            isLast={i === explorer.bookmarks().length - 1}
            sortFileKey={explorer.sortFileKey}
            sortFileAsc={explorer.sortFileAsc}
          />
        ))
      }
    </div>
  )
}

export { BookmarksView }