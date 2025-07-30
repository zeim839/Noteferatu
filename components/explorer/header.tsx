import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { useExplorerContext, SortFileKey, ViewType } from "./context"

import {
  ChevronRightIcon,
  ChevronDownIcon,
  ArrowDownWideNarrowIcon,
  BookmarkIcon,
  FilesIcon,
  GroupIcon,
} from "lucide-react"

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/core/select"

import {
  Popover,
  PopoverTrigger,
  PopoverContent
} from "@/components/core/popover"

function ExplorerHeader() {
  const [isSelectDocOpen, setIsSelectDocOpen] = React.useState<boolean>(false)
  const explorer = useExplorerContext()

  return (
    <Sidebar.Header className="flex flex-row justify-between items-center px-1 min-h-[29px]">
      <Select
        defaultOpen={false}
        onOpenChange={(open) => setIsSelectDocOpen(open)}
        onValueChange={(value) => explorer.setView(value as ViewType)}
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
            {explorer.view() === "documents" ? "Documents" : explorer.view() === "tags" ? "Tags" : "Bookmarks"}
          </p>
        </SelectTrigger>
        <SelectContent className="bg-[#EDF0F4]">
          <SelectItem value="documents">
            <FilesIcon className="size-3" />
            <span>Documents</span>
          </SelectItem>
          <SelectItem value="bookmarks">
            <BookmarkIcon className="size-3" />
            <span>Bookmarks</span>
          </SelectItem>
          <SelectItem value="tags">
            <GroupIcon className="size-3" />
            <span>Tags</span>
          </SelectItem>
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
  )
}

export { ExplorerHeader }
