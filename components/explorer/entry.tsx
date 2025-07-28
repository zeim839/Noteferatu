import * as React from "react"
import { Button } from "@/components/core/button"
import { File, copyFile, removeFile, moveFile } from "@/lib/helsync"

import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
  ContextMenuSeparator,
  ContextMenuSub,
  ContextMenuSubTrigger,
  ContextMenuSubContent,
} from "@/components/core/context-menu"

import {
  FolderIcon,
  NotepadTextIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  Trash2Icon,
  FilePenLineIcon,
  FilesIcon,
  BookmarkIcon,
  SquareArrowOutUpRightIcon,
  CirclePlusIcon,
  ShareIcon,
  CheckIcon,
  XIcon,
} from "lucide-react"

// Extend File to include children
export interface FileEntry extends File {
  children?: FileEntry[]
}

type EntryProps = {
  file: FileEntry
  depth?: number
  expandedFolders: Set<string>
  setExpandedFolders: (fn: (prev: Set<string>) => Set<string>) => void
  isLast?: boolean
  sortFileKey: () => "name" | "createdAt" | "modifiedAt"
  sortFileAsc: () => boolean
}

// File explorer entry.
export function Entry({
  file,
  depth = 0,
  expandedFolders,
  setExpandedFolders,
  isLast = true,
  sortFileKey,
  sortFileAsc,
}: EntryProps) {
  const [isBeingRenamed, setIsBeingRenamed] = React.useState<boolean>(false)
  const [newName, setNewName] = React.useState(file.name)
  const inputRef = React.useRef<HTMLInputElement>(null)

  // When clicking "rename", this focuses the user's cursor on
  // the input field.
  React.useEffect(() => {
    if (isBeingRenamed) {
      setTimeout(() => {
        inputRef.current?.focus()
        inputRef.current?.select()
      }, 0)
    }
  }, [isBeingRenamed])

  // Renames a file by moving it to the same parent but with
  // a different name.
  const handleRename = () => {
    if (newName.trim() && newName !== file.name) {
      moveFile(file.id.toString(), file.parent?.toString(), newName)
    }
    setIsBeingRenamed(false)
  }

  // Cancels the rename operation.
  const cancelRename = () => {
    setIsBeingRenamed(false)
    setNewName(file.name)
  }

  const entryId = file.id.toString()
  const isExpanded = expandedFolders.has(entryId)
  const hasChildren = file.children && file.children.length > 0

  // Expand/retract a folder object.
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

  // Icon varies depending on whether file object is a file or folder.
  const icon = file.isFolder ?
    <FolderIcon strokeWidth={1.6} className="h-[15px]" /> :
    <NotepadTextIcon strokeWidth={1.6} className="h-[15px]" />

  // Show a chevron for toggling folder expansion.
  const chevronIcon = hasChildren ? (
    isExpanded ?
      (<ChevronDownIcon className="size-4" strokeWidth={1.6} />) :
      (<ChevronRightIcon className="size-4" strokeWidth={1.6} />)
  ) : null

  // Comparison function used to implement folder child sorting.
  const compareFn = (a: FileEntry, b: FileEntry): number => {
    const [valueA, valueB] = [a[sortFileKey()], b[sortFileKey()]]
    if (valueA < valueB) {
      return sortFileAsc() ? -1 : 1
    }
    if (valueA > valueB) {
      return sortFileAsc() ? 1 : -1
    }
    return 0
  }

  return (
    <>
      <ContextMenu>
        <ContextMenuTrigger>
          <div
            data-is-being-renamed={isBeingRenamed}
            className="relative grid grid-cols-[20px_auto_20px] data-[is-being-renamed=true]:grid-cols-[20px_auto_50px] items-center font-light text-sm hover:bg-[#DCE0E8] hover:rounded-sm gap-2 h-[32px]"
            style={{ paddingLeft: `${depth * 16}px` }}
            onClick={() => {
              if (hasChildren && !isBeingRenamed) {
                toggleExpanded()
              }
            }}
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
            {isBeingRenamed ? (
              <input
                ref={inputRef}
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleRename()
                  if (e.key === "Escape") cancelRename()
                }}
                onBlur={cancelRename}
                className="px-1 rounded-sm text-sm h-[22px] w-full"
              />
            ) : (
              <p className="max-h-[17px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
                {file.name}
              </p>
            )}
            {
              (isBeingRenamed) ?
                <div className="flex gap-1 items-center justify-center">
                  <Button
                    onMouseDown={(e) => e.preventDefault()}
                    onClick={handleRename}
                    variant="confirmation"
                    size="icon"
                    className="p-0 m-0 size-4"
                  >
                    <CheckIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
                  </Button>
                  <Button
                    onMouseDown={(e) => e.preventDefault()}
                    onClick={cancelRename}
                    variant="destructive"
                    size="icon"
                    className="p-0 m-0 size-4"
                  >
                    <XIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
                  </Button>
                </div> : chevronIcon
            }
          </div>
        </ContextMenuTrigger>
        <ContextMenuContent className="text-xs">
          {
            (file.isFolder) ?
              <ContextMenuItem onSelect={() => {console.log("new file", file.id)}}>
                <CirclePlusIcon className="size-3" />
                <span>New File</span>
              </ContextMenuItem> :
              <ContextMenuItem onSelect={() => {console.log("open", file.id)}} disabled>
                <SquareArrowOutUpRightIcon className="size-3" />
                <span>Open</span>
              </ContextMenuItem>
          }
          <ContextMenuSub>
            <ContextMenuSubTrigger disabled>
              <ShareIcon className="size-3" />
              <span>Export As</span>
            </ContextMenuSubTrigger>
            <ContextMenuSubContent>
              <ContextMenuItem>
                TODO
              </ContextMenuItem>
            </ContextMenuSubContent>
          </ContextMenuSub>
          <ContextMenuSeparator />
          <ContextMenuItem onSelect={() => setIsBeingRenamed(true)}>
            <FilePenLineIcon className="size-3" />
            <span>Rename</span>
          </ContextMenuItem>
          <ContextMenuItem onSelect={() => {
            copyFile(file.id.toString(), file.parent?.toString())
          }}>
            <FilesIcon className="size-3" />
            <span>Duplicate</span>
          </ContextMenuItem>
          <ContextMenuItem onSelect={() => {console.log("Tags", file.id)}}>
            <BookmarkIcon className="size-3" />
            <span>Manage Tags</span>
          </ContextMenuItem>
          <ContextMenuSeparator />
          <ContextMenuItem onSelect={() => {
            removeFile(file.id.toString())
          }}>
            <Trash2Icon className="size-3" />
            <span>Delete</span>
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>

      {/* Render children if expanded */}
      {isExpanded && file.children && (
        <>
          {[...file.children].sort(compareFn).map((child, i) => (
            <Entry
              key={child.id}
              file={child}
              depth={depth + 1}
              expandedFolders={expandedFolders}
              setExpandedFolders={setExpandedFolders}
              isLast={i === file.children!.length - 1}
              sortFileKey={sortFileKey}
              sortFileAsc={sortFileAsc}
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
