"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { useExplorerContext, SortFileKey } from "./explorer"
import { Entry, FileEntry } from "./entry"
import { ArrowDownWideNarrowIcon, ChevronRightIcon } from "lucide-react"

import {
  Popover,
  PopoverTrigger,
  PopoverContent
} from "@/components/core/popover"

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/core/select"

// File explorer shows a tree of all available directories, documents,
// and their subchildren.
function ExplorerPanel() {
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())
  const { documents, sortFileKey, setSortFileKey, sortFileAsc, setSortFileAsc } =
    useExplorerContext()

  // Compares two file entries (used for sorting).
  const compareFn = (a: FileEntry, b: FileEntry): number => {
    const [keyA, keyB] = [a[sortFileKey()], b[sortFileKey()]]
    const asc = sortFileAsc()
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
        <Button
          tooltip="Customize View"
          tooltipSide="bottom"
          variant="outline"
          className="p-2 rounded-md h-6 px-1 flex items-center justify-between pr-2"
        >
          <ChevronRightIcon strokeWidth={1.6} />
          <p className="text-xs max-h-[15px] max-w-[150px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
            Documents
          </p>
        </Button>
        <div className="flex flex-row">
          <Popover>
            <PopoverTrigger asChild>
              <Button variant="ghost" size="icon" tooltip="Sort">
                <ArrowDownWideNarrowIcon strokeWidth={1.6} />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[150px] p-3 flex flex-col gap-2">
              <Select
                defaultValue={sortFileKey() as string}
                onValueChange={(value) => setSortFileKey(value as SortFileKey)}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Sort By"/>
                </SelectTrigger>
                <SelectContent className="bg-[#EDF0F4]">
                  <SelectItem value="name">File Name</SelectItem>
                  <SelectItem value="createdAt">Created</SelectItem>
                  <SelectItem value="modifiedAt">Last modified</SelectItem>
                </SelectContent>
              </Select>
              <Select
                defaultValue={sortFileAsc() ? "asc" : "des"}
                onValueChange={(value) => setSortFileAsc(value === "asc")}
              >
                <SelectTrigger className="w-full">
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
        {[...documents()].sort(compareFn).map((doc, i) => (
          <Entry
            key={doc.id}
            file={doc}
            expandedFolders={expandedFolders}
            setExpandedFolders={setExpandedFolders}
            isLast={i === documents.length - 1}
            sortFileKey={sortFileKey}
            sortFileAsc={sortFileAsc}
          />
        ))}
      </div>
    </div>
  )
}

export { ExplorerPanel }
