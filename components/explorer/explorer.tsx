"use client"

import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import * as React from "react"

interface DocumentEntry {
  title: string
  type: "document" | "folder"
  children?: DocumentEntry[]
}

import {
  FolderIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  ArrowDownWideNarrowIcon,
  SlidersHorizontalIcon,
  NotepadTextIcon,
} from "lucide-react"

const sampleDocuments: DocumentEntry[] = [
  {
    title: "Introduction",
    type: "document",
  },
  {
    title: "NoteFeratu Tutorial",
    type: "document",
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
    title: "Politics",
    type: "folder",
    children: [
      {
        title: "US Elections",
        type: "document",
      },
      {
        title: "Foreign Policy",
        type: "document",
      },
    ],
  },
  {
    title: "Travel & Outdoors",
    type: "folder",
    children: [
      {
        title: "Hiking Trails",
        type: "document",
      },
      {
        title: "National Parks",
        type: "folder",
        children: [
          {
            title: "Yellowstone",
            type: "document",
          },
          {
            title: "Yosemite",
            type: "document",
          },
          {
            title: "Super long title name",
            type: "document",
          },
        ],
      },
    ],
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
]

function Entry({
  title = "Untitled",
  type,
  subEntries = [],
  depth = 0,
  expandedFolders,
  setExpandedFolders,
  isLast = true,
}: {
  title?: string
  type: string
  subEntries?: DocumentEntry[]
  depth?: number
  expandedFolders: Set<string>
  setExpandedFolders: (fn: (prev: Set<string>) => Set<string>) => void
  isLast?: boolean
}) {
  const entryId = `${depth}-${title}`
  const isExpanded = expandedFolders.has(entryId)
  const hasChildren = subEntries && subEntries.length > 0

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

  const icon =
    type === "document" ? (
      <NotepadTextIcon strokeWidth={1.6} className="h-[15px]" />
    ) : (
      <FolderIcon strokeWidth={1.6} className="h-[15px]" />
    )

  const chevronIcon = hasChildren ? (
    isExpanded ? (
      <ChevronDownIcon strokeWidth={1.6} />
    ) : (
      <ChevronRightIcon strokeWidth={1.6} />
    )
  ) : null

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
          {title}
        </p>
        {hasChildren ? (
          <Button variant="ghost" size="icon" onClick={toggleExpanded}>
            {chevronIcon}
          </Button>
        ) : null}
      </div>

      {/* Render children if expanded */}
      {isExpanded && hasChildren && (
        <>
          {subEntries.map((child, i) => (
            <Entry
              key={`${entryId}-${i}`}
              title={child.title}
              type={child.type}
              subEntries={child.children}
              depth={depth + 1}
              expandedFolders={expandedFolders}
              setExpandedFolders={setExpandedFolders}
              isLast={i === subEntries.length - 1}
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
                height: `${subEntries.length * 32}px`,
                zIndex: -1,
              }}
            />
          )}
        </>
      )}
    </>
  )
}

function Explorer() {
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(
    new Set(),
  )

  return (
    <div className="w-full max-h-[calc(100vh-35px)] min-w-[200px] flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 min-h-[29px]">
        <div className="flex flex-row items-center gap-1">
          <Button variant="ghost" size="icon">
            <FolderIcon strokeWidth={1.6} />
          </Button>
          <p className="text-xs">Documents</p>
          <Button variant="ghost" size="icon">
            <ChevronDownIcon strokeWidth={1.6} />
          </Button>
        </div>
        <div className="flex flex-row">
          <Button variant="ghost" size="icon" tooltip="Filter / Sort">
            <ArrowDownWideNarrowIcon strokeWidth={1.6} />
          </Button>
          <Button variant="ghost" size="icon" tooltip="Customize View">
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>
      <div className="w-full flex flex-col px-1 pt-1 flex-1 overflow-auto scrollbar-hide relative">
        {sampleDocuments.map((obj, i) => (
          <Entry
            key={i}
            title={obj.title}
            type={obj.type}
            subEntries={obj.children}
            expandedFolders={expandedFolders}
            setExpandedFolders={setExpandedFolders}
            isLast={i === sampleDocuments.length - 1}
          />
        ))}
      </div>
    </div>
  )
}

export { Explorer }
