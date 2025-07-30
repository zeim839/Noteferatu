import * as React from "react"
import { useExplorerContext, SortFileKey } from "../context"
import { FileNameExistsError } from "./utils"

import {
  createFile,
  HelsyncError,
  createFolder
} from "@/lib/helsync"

import {
  ArrowDownWideNarrowIcon,
  CirclePlusIcon,
  LayoutTemplateIcon,
  FilePlusIcon,
  FolderPenIcon,
  FilesIcon,
  BookmarkIcon,
  ShareIcon,
  ListOrderedIcon,
  ArrowDownAZIcon,
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
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
} from "@/components/core/context-menu"

function ExplorerContextMenu({ children, ...props }: React.ComponentProps<"div">) {
  const explorer = useExplorerContext()

  // Creates a new file in the root directory with the name `Untitled`.
  // If `Untitled` exists, it tries once more with a timestamp suffix.
  const onCreateFile = () => {
    createFile(`Untitled`)
      .catch((err: HelsyncError) => {
        if (err.error.type !== "database") {
          throw err
        }
        if (err.error.error !== FileNameExistsError) {
          throw err
        }
        const timestamp = new Date().toISOString()
        createFile(`Untitled ${timestamp}`)
      })
  }

  // Creates a new folder in the root directory with the name `Untitled
  // Folder`. If `Untitled Folder` exists, it tries once more with a
  // timestamp suffix.
  const onCreateFolder = () => {
    createFolder(`Untitled Folder`)
      .catch((err: HelsyncError) => {
        if (err.error.type !== "database") {
          throw err
        }
        if (err.error.error !== FileNameExistsError) {
          throw err
        }
        const timestamp = new Date().toISOString()
        createFolder(`Untitled Folder ${timestamp}`)
      })
  }

  return (
    <ContextMenu>
      <ContextMenuTrigger {...props}>
        { children }
      </ContextMenuTrigger>
      <ContextMenuContent className="text-xs">
        <ContextMenuSub>
          <ContextMenuSubTrigger>
            <CirclePlusIcon className="size-3" />
            <span>New</span>
          </ContextMenuSubTrigger>
          <ContextMenuSubContent>
            <ContextMenuItem onSelect={onCreateFile}>
              <FilePlusIcon className="size-3" />
              <span>File</span>
            </ContextMenuItem>
            <ContextMenuItem onSelect={onCreateFolder}>
              <FolderPenIcon className="size-3" />
              <span>Folder</span>
            </ContextMenuItem>
          </ContextMenuSubContent>
        </ContextMenuSub>
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
        <ContextMenuSub>
          <ContextMenuSubTrigger>
            <LayoutTemplateIcon className="size-3" />
            <span>Filter</span>
          </ContextMenuSubTrigger>
          <ContextMenuSubContent>
            <ContextMenuRadioGroup
              value={(explorer.isViewDocuments()) ? "documents" : "tags"}
              onValueChange={
                (value) => explorer.setIsViewDocuments(value === "documents")
              }
            >
              <ContextMenuRadioItem value="documents">
                <FilesIcon className="size-3" />
                <span>Documents</span>
              </ContextMenuRadioItem>
              <ContextMenuRadioItem value="tags">
                <BookmarkIcon className="size-3" />
                <span>Tags</span>
              </ContextMenuRadioItem>
            </ContextMenuRadioGroup>
          </ContextMenuSubContent>
        </ContextMenuSub>
        <ContextMenuSub>
          <ContextMenuSubTrigger>
            <ArrowDownWideNarrowIcon className="size-3" />
            <span>Sort</span>
          </ContextMenuSubTrigger>
          <ContextMenuSubContent>
            <ContextMenuSub>
              <ContextMenuSubTrigger>
                <ArrowDownAZIcon className="size-3" />
                <span>Sort by</span>
              </ContextMenuSubTrigger>
              <ContextMenuSubContent>
                <ContextMenuRadioGroup
                  value={explorer.sortFileKey()}
                  onValueChange={
                    (value) => explorer.setSortFileKey(value as SortFileKey)
                  }
                >
                  <ContextMenuRadioItem value="name">
                    <span>File Name</span>
                  </ContextMenuRadioItem>
                  <ContextMenuRadioItem value="createdAt">
                    <span>Date Created</span>
                  </ContextMenuRadioItem>
                  <ContextMenuRadioItem value="modifiedAt">
                    <span>Date Modified</span>
                  </ContextMenuRadioItem>
                </ContextMenuRadioGroup>
              </ContextMenuSubContent>
            </ContextMenuSub>
            <ContextMenuSub>
              <ContextMenuSubTrigger>
                <ListOrderedIcon className="size-3" />
                <span>Order by</span>
              </ContextMenuSubTrigger>
              <ContextMenuSubContent>
                <ContextMenuRadioGroup
                  value={(explorer.sortFileAsc()) ? "asc" : "des"}
                  onValueChange={
                    (value) => explorer.setSortFileAsc(value === "asc")
                  }
                >
                  <ContextMenuRadioItem value="asc">
                    <span>Ascending</span>
                  </ContextMenuRadioItem>
                  <ContextMenuRadioItem value="des">
                    <span>Descending</span>
                  </ContextMenuRadioItem>
                </ContextMenuRadioGroup>
              </ContextMenuSubContent>
            </ContextMenuSub>
          </ContextMenuSubContent>
        </ContextMenuSub>
      </ContextMenuContent>
    </ContextMenu>
  )
}

export { ExplorerContextMenu }
