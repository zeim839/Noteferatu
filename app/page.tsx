"use client"

import { Window, WindowProvider } from "@/components/window/window"
import { Button } from "@/components/core/button"
import { BufferGroup } from "@/components/buffer/group"
import { useState } from "react"

import { ExplorerProvider } from "@/components/explorer/context"
import { Explorer } from "@/components/explorer/explorer"

import { AgentProvider } from "@/components/agent/context"
import { Agent } from "@/components/agent/agent"

import {
  PanelLeftDashedIcon,
  SettingsIcon,
  MessageSquareTextIcon,
  SearchIcon,
  FolderSyncIcon,
  CirclePlusIcon,
} from "lucide-react"

export default function Home() {
  const [isLeftSidebarOpen, setLeftSidebarOpen] = useState<boolean>(false)
  const [isRightSidebarOpen, setRightSidebarOpen] = useState<boolean>(false)
  return (
    <WindowProvider
      isLeftSidebarOpen={isLeftSidebarOpen}
      isRightSidebarOpen={isRightSidebarOpen}
      setLeftSidebarOpen={setLeftSidebarOpen}
      setRightSidebarOpen={setRightSidebarOpen}
    >
      <Window>
        <Window.Titlebar>
          <Window.Titlebar.ToolGroup>
            <Button variant="ghost" size="icon" tooltip="File Explorer" onClick={() => setLeftSidebarOpen(!isLeftSidebarOpen)}>
              <PanelLeftDashedIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Cloud Sync">
              <FolderSyncIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Search & Command">
              <SearchIcon strokeWidth={1.6} />
            </Button>
          </Window.Titlebar.ToolGroup>
          <Window.Titlebar.ToolGroup>
            <Button variant="ghost" size="icon" tooltip="New Document">
              <CirclePlusIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Agent Panel" onClick={() => setRightSidebarOpen(!isRightSidebarOpen)}>
              <MessageSquareTextIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Settings">
              <SettingsIcon strokeWidth={1.6} />
            </Button>
          </Window.Titlebar.ToolGroup>
        </Window.Titlebar>
        <Window.LeftSidebar>
          <ExplorerProvider>
            <Explorer />
          </ExplorerProvider>
        </Window.LeftSidebar>
        <Window.Content>
          <BufferGroup />
        </Window.Content>
        <Window.RightSidebar>
          <AgentProvider>
            <Agent />
          </AgentProvider>
        </Window.RightSidebar>
      </Window>
    </WindowProvider>
  )
}
