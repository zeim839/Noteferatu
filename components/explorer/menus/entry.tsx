import * as React from "react"
import { Button } from "@/components/core/button"
import { FileNameExistsError } from "./utils"

import {
  FileEntry,
  copyFile,
  removeFile,
  createFile,
  HelsyncError,
  createFolder
} from "@/lib/helsync"

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

import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/core/dialog"

interface EntryContextMenuProps extends React.ComponentProps<"div"> {
  file: FileEntry
  setIsBeingRenamed: (value: boolean) => void,
}

function EntryContextMenu({ file, setIsBeingRenamed, children, ...props} : EntryContextMenuProps) {
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = React.useState(false)

  // This function is called when the dialog's open state changes.
  // We use it to manage the visibility of the delete dialog.
  const handleDialogOpenChange = (open: boolean) => {
    setIsDeleteDialogOpen(open)
  }

  // Attempts to duplicate the file. It appends `Copy` to the end of the
  // file name. If the file name is still taken, it adds a timestamp
  // suffix and tries once more.
  const onDuplicate = () => {
    copyFile(file.id.toString(), file.parent?.toString(), `${file.name} Copy`)
      .catch((err: HelsyncError) => {
        if (err.error.type !== "database") {
          throw err
        }
        if (err.error.error !== FileNameExistsError) {
          throw err
        }
        const timestamp = new Date().toISOString()
        const fileName = `${file.name} Copy ${timestamp}`
        copyFile(file.id.toString(), file.parent?.toString(), fileName)
      })
  }

  // Creates a new file. Tries the `untitled` file name. If it's already
  // taken, it tries once more by appending a timestamp suffix to the new
  // file name.
  //
  // Files can only be created if Entry is a folder.
  const onCreateFile = () => {
    createFile(`Untitled`, file.id.toString())
      .catch((err: HelsyncError) => {
        if (err.error.type !== "database") {
          throw err
        }
        if (err.error.error !== FileNameExistsError) {
          throw err
        }
        const timestamp = new Date().toISOString()
        createFile(`Untitled ${timestamp}`, file.id.toString())
      })
  }

  // Creates a new folder. Tries the `untitled` file name. If it's already
  // taken, it tries once more by appending a timestamp suffix to the new
  // file name.
  //
  // Folders can only be created if Entry is a folder.
  const onCreateFolder = () => {
    createFolder(`Untitled Folder`, file.id.toString())
      .catch((err: HelsyncError) => {
        if (err.error.type !== "database") {
          throw err
        }
        if (err.error.error !== FileNameExistsError) {
          throw err
        }
        const timestamp = new Date().toISOString()
        createFolder(`Untitled Folder ${timestamp}`, file.id.toString())
      })
  }

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
                <ContextMenuItem onSelect={onCreateFile}>
                  <FilePlusIcon className="size-3" />
                  <span>File</span>
                </ContextMenuItem>
                <ContextMenuItem onSelect={onCreateFolder}>
                  <FolderPenIcon className="size-3" />
                  <span>Folder</span>
                </ContextMenuItem>
              </ContextMenuSubContent>
            </ContextMenuSub> :
            <ContextMenuItem disabled>
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
        <ContextMenuItem onSelect={onDuplicate}>
          <FilesIcon className="size-3" />
          <span>Duplicate</span>
        </ContextMenuItem>
        <ContextMenuItem>
          <BookmarkIcon className="size-3" />
          <span>Manage Tags</span>
        </ContextMenuItem>
        <ContextMenuSeparator />
        <Dialog open={isDeleteDialogOpen} onOpenChange={handleDialogOpenChange}>
          <DialogTrigger asChild>
            <ContextMenuItem onSelect={(e) => {
              e.preventDefault()
              e.stopPropagation()
              setIsDeleteDialogOpen(true)
            }}>
              <Trash2Icon className="size-3" />
              <span>Delete</span>
            </ContextMenuItem>
          </DialogTrigger>
          <DialogContent className="w-100" showCloseButton={false}>
            <DialogHeader>
              <DialogTitle>
                Delete File &quot;{file.name.length > 20 ? file.name.substring(0, 18) + "..." : file.name}&quot;?
              </DialogTitle>
              <DialogDescription>
                Are you sure you want to delete this file? This action
                cannot be undone.
              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <DialogClose asChild>
                <Button onClick={(e) => {
                  e.stopPropagation()
                  setIsDeleteDialogOpen(false)
                }}>Cancel</Button>
              </DialogClose>
              <Button variant="destructive" onClick={() => {
                removeFile(file.id.toString())
                setIsDeleteDialogOpen(false)
              }}>Delete</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </ContextMenuContent>
    </ContextMenu>
  )
}

export { EntryContextMenu }
