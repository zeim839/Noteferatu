import * as React from "react"
import { useExplorerContext } from "../context"
import { FileEntry } from "@/lib/helsync"
import { Entry } from "../entry"

// Displays a list of tags sorted in a particular order when the
// explorer view is set to 'tags'.
function TagsView() {
  const explorer = useExplorerContext()
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())

  return null
}

export { TagsView }
