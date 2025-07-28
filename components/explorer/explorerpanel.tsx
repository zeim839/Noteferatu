"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { useExplorerContext, SortFileKey } from "./explorer"
import { Entry, FileEntry } from "./entry"

import {
  ArrowDownWideNarrowIcon,
  ChevronRightIcon,
  ChevronDownIcon,
} from "lucide-react"

import {
  Popover,
  PopoverTrigger,
  PopoverContent
} from "@/components/core/popover"

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/core/select"

// File explorer shows a tree of all available directories, documents,
// and their subchildren.
function ExplorerPanel() {
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())
  const [isSelectDocOpen, setIsSelectDocOpen] = React.useState<boolean>(false)
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
    <div className="w-full max-h-[calc(100vh-35px)] min-w-[200px] flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 min-h-[29px]">
        <Select
          defaultOpen={false}
          onOpenChange={(open) => setIsSelectDocOpen(open)}
          onValueChange={(value) => explorer.setIsViewDocuments(value === "documents")}
        >
          <SelectTrigger
            withIcon={false} size="sm"
            className="data-[size=sm]:h-6 p-2 px-1 border-none rounded-sm hover:bg-[#D4D8E1] flex items-center shadow-none pr-2"
          >
            {isSelectDocOpen ? (
              <ChevronDownIcon strokeWidth={1.6} />
            ) : (
              <ChevronRightIcon strokeWidth={1.6} />
            )}
            <p className="text-xs max-h-[15px]">
              {(explorer.isViewDocuments()) ? "Documents" : "Tags"}
            </p>
          </SelectTrigger>
          <SelectContent className="bg-[#EDF0F4]">
            <SelectItem value="documents">Documents</SelectItem>
            <SelectItem value="tags">Tags</SelectItem>
          </SelectContent>
        </Select>
        <div className="flex flex-row">
          <Popover>
            <PopoverTrigger asChild>
              <Button variant="ghost" size="icon" tooltip="Sort">
                <ArrowDownWideNarrowIcon strokeWidth={1.6} />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[130px] p-2 flex flex-col gap-2">
              <Select
                defaultValue={explorer.sortFileKey() as string}
                onValueChange={(value) => explorer.setSortFileKey(value as SortFileKey)}
              >
                <SelectTrigger className="w-full text-xs" size="sm">
                  <SelectValue placeholder="Sort By"/>
                </SelectTrigger>
                <SelectContent className="bg-[#EDF0F4]">
                  <SelectItem value="name">File Name</SelectItem>
                  <SelectItem value="createdAt">Date Created</SelectItem>
                  <SelectItem value="modifiedAt">Date Modified</SelectItem>
                </SelectContent>
              </Select>
              <Select
                defaultValue={explorer.sortFileAsc() ? "asc" : "des"}
                onValueChange={(value) => explorer.setSortFileAsc(value === "asc")}
              >
                <SelectTrigger className="w-full text-xs" size="sm">
                  <SelectValue placeholder="Order"/>
                </SelectTrigger>
                <SelectContent className="bg-[#EDF0F4]">
                  <SelectItem value="asc">Ascending</SelectItem>
                  <SelectItem value="des">Descending</SelectItem>
                </SelectContent>
              </Select>
            </PopoverContent>
          </Popover>
        </div>
      </Sidebar.Header>
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
    </div>
  )
}

export { ExplorerPanel }
