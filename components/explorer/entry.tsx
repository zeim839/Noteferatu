import { File } from "@/lib/helsync"
import { Button } from "@/components/core/button"

import {
  FolderIcon,
  NotepadTextIcon,
  ChevronDownIcon,
  ChevronRightIcon,
} from "lucide-react"

// Extend File to include children
export interface FileEntry extends File {
  children?: FileEntry[]
}

type EntryProps = {
  file: FileEntry,
  depth?: number,
  expandedFolders: Set<string>,
  setExpandedFolders: (fn: (prev: Set<string>) => Set<string>) => void
  isLast?: boolean,
}

// File explorer entry.
export function Entry({
  file,
  depth = 0,
  expandedFolders,
  setExpandedFolders,
  isLast = true
}: EntryProps) {

  const entryId = file.id.toString()
  const isExpanded = expandedFolders.has(entryId)
  const hasChildren = file.children && file.children.length > 0
  const toggleExpanded = () => {
    setExpandedFolders((prev) => {
      const newSet = new Set(prev)
      if (newSet.has(entryId)) {
        newSet.delete(entryId)
      } else {
        newSet.add(entryId)
      }
      return newSet
    })
  }

  const icon = file.isFolder ?
    (<FolderIcon strokeWidth={1.6} className="h-[15px]" />) :
    (<NotepadTextIcon strokeWidth={1.6} className="h-[15px]" />)

  const chevronIcon = hasChildren ? (isExpanded ?
    (<ChevronDownIcon strokeWidth={1.6} />) :
    (<ChevronRightIcon strokeWidth={1.6} />)) :
    null

  return (
    <>
      <div
        className="relative grid grid-cols-[20px_auto_30px] items-center font-light text-sm hover:bg-[#DCE0E8] hover:rounded-md gap-2 h-[32px]"
        style={{ paddingLeft: `${depth * 16}px` }}
      >
        {/* Connecting lines */}
        {depth > 0 && (
          <>
            {/* Vertical line from parent */}
            <div
              className="absolute bg-gray-300"
              style={{
                left: `${(depth - 1) * 16 + 10}px`,
                top: 0,
                width: "1px",
                height: isLast ? "16px" : "32px",
              }}
            />
            {/* Horizontal line to icon */}
            <div
              className="absolute bg-gray-300"
              style={{
                left: `${(depth - 1) * 16 + 10}px`,
                top: "15px",
                width: `${16}px`,
                height: "1px",
              }}
            />
          </>
        )}
        {icon}
        <p className="max-h-[17px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
          {file.name}
        </p>
        {hasChildren ? (
          <Button variant="ghost" size="icon" onClick={toggleExpanded}>
            {chevronIcon}
          </Button>
        ) : null}
      </div>

      {/* Render children if expanded */}
      {isExpanded && file.children && (
        <>
          {file.children.map((child, i) => (
            <Entry
              key={child.id}
              file={child}
              depth={depth + 1}
              expandedFolders={expandedFolders}
              setExpandedFolders={setExpandedFolders}
              isLast={i === file.children!.length - 1}
            />
          ))}
          {/* Vertical line continuation for non-last folders */}
          {!isLast && depth > 0 && (
            <div
              className="absolute bg-gray-300"
              style={{
                left: `${(depth - 1) * 16 + 10}px`,
                top: "32px",
                width: "1px",
                height: `${file.children.length * 32}px`,
                zIndex: -1,
              }}
            />
          )}
        </>
      )}
    </>
  )
}
