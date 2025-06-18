"use client"

import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import * as React from "react"

import {
  FolderClosedIcon,
  ChevronDownIcon,
  ArrowDownWideNarrowIcon,
  SlidersHorizontalIcon,
  BookTextIcon,
  LockIcon,
  GlobeIcon,
} from "lucide-react"

const sampleDocuments = [
  {
    title: "Introduction",
    type: "document"
  },
  {
    title: "NoteFeratu Tutorial",
    type: "document"
  },
  {
    title: "Roman Empire",
    type: "folder",
  },
  {
    title: "First Order Theory",
    type: "document",
  },
  {
    title: "Passwords",
    type: "encrypted",
  },
  {
    title: "Coursework",
    type: "document",
  },
  {
    title: "Recipes",
    type: "document",
  },
  {
    title: "Diagonalization Proof",
    type: "document",
  },
  {
    title: "Campaigns of Napoleon",
    type: "document",
  },
  {
    title: "Markdown",
    type: "document",
  },
  {
    title: "Siege of Toulon",
    type: "document",
  },
  {
    title: "Battle of Pharsalus",
    type: "document",
  },
  {
    title: "Battle of Cannae",
    type: "document",
  },
  {
    title: "Second Punic War",
    type: "document",
  },
  {
    title: "www.unqualified-reservations.org",
    type: "website"
  },
]

function DocEntry({ title="Untitled", type }:
{ title?: string, type: string }) {
  const icon = (type === "document") ?
    <BookTextIcon strokeWidth={1.6} className="w-[16px] h-[16px]" /> :
    (type === "folder") ?
      <FolderClosedIcon strokeWidth={1.6} className="w-[19px]" /> :
      (type === "encrypted") ?
        <LockIcon strokeWidth={1.6} className="w-[17px]" /> :
        <GlobeIcon strokeWidth={1.6} className="h-[16px]" />

  return (
    <div className="grid grid-cols-[20px_auto_20px] items-center p-2 font-light text-sm hover:bg-[#DCE0E8] hover:rounded-md gap-2">
      {icon}
      <p className="text-nowrap text-ellipsis overflow-hidden">
        {title}
      </p>
      {
        (type === "folder") ? (
          <Button variant="ghost" size="icon">
            <ChevronDownIcon strokeWidth={1.6} />
          </Button>
        ) : null
      }
    </div>
  )
}

function Explorer() {
  return (
    <div className="w-full min-w-[200px] h-full flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1">
        <div className="flex flex-row items-center gap-1">
          <Button variant="ghost" size="icon">
            <FolderClosedIcon strokeWidth={1.6} />
          </Button>
          <p className="text-xs">Documents</p>
          <Button variant="ghost" size="icon">
            <ChevronDownIcon strokeWidth={1.6} />
          </Button>
        </div>
        <div className="flex flex-row">
          <Button variant="ghost" size="icon" tooltip="Filter & Sort">
            <ArrowDownWideNarrowIcon strokeWidth={1.6} />
          </Button>
          <Button variant="ghost" size="icon" tooltip="Customize View">
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>
      <div className="w-full flex flex-col px-1 pt-2">
        {
          sampleDocuments.map((obj, i) => (
            <DocEntry key={i} title={obj.title} type={obj.type} />
          ))
        }
      </div>
    </div>
  )
}

export { Explorer }
