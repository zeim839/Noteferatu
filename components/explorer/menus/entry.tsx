import * as React from "react"
import { FileEntry, copyFile, removeFile } from "@/lib/helsync"

import {
  Trash2Icon,
  FilePenLineIcon,
  FilesIcon,
  BookmarkIcon,
  SquareArrowOutUpRightIcon,
  CirclePlusIcon,
  ShareIcon,
  FilePlusIcon,
  FolderPenIcon,
} from "lucide-react"

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

interface EntryContextMenuProps extends React.ComponentProps<"div"> {
  file: FileEntry
  setIsBeingRenamed: (value: boolean) => void,
}

function EntryContextMenu({ file, setIsBeingRenamed, children, ...props} : EntryContextMenuProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger {...props}>
        { children }
      </ContextMenuTrigger>
      <ContextMenuContent className="text-xs">
        {
          (file.isFolder) ?
            <ContextMenuSub>
              <ContextMenuSubTrigger>
                <CirclePlusIcon className="size-3" />
                <span>New</span>
              </ContextMenuSubTrigger>
              <ContextMenuSubContent>
                <ContextMenuItem>
                  <FilePlusIcon className="size-3" />
                  <span>File</span>
                </ContextMenuItem>
                <ContextMenuItem>
                  <FolderPenIcon className="size-3" />
                  <span>Folder</span>
                </ContextMenuItem>
              </ContextMenuSubContent>
            </ContextMenuSub> :
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
          copyFile(file.id.toString(), file.parent?.toString(), `${file.name} Copy`)
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
  )
}

export { EntryContextMenu }
