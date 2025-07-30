"use client"

import * as React from "react"
import { ExplorerHeader } from "./header"
import { ExplorerContextMenu } from "./menus/explorer"

import { DocumentsView } from "./views/documents"
import { BookmarksView } from "./views/bookmarks"
import { TagsView } from "./views/tags"

// File explorer shows a tree of all available directories, documents,
// and their subchildren.
function Explorer() {
  return (
    <ExplorerContextMenu className="w-full max-h-[calc(100vh-35px)] h-[calc(100vh-35px)] min-w-[200px] flex flex-col">
      <ExplorerHeader />
      <DocumentsView />
      <BookmarksView />
      <TagsView />
    </ExplorerContextMenu>
  )
}

export { Explorer }
