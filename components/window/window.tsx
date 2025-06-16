"use client"

import { Titlebar, Toolbar } from "./titlebar"
import { Button } from "@/components/ui/button"
import { TooltipProvider } from "@/components/ui/tooltip"
import { Sidebars } from "./sidebars"
import { cn } from "@/lib/utils"
import { ComponentProps, ReactElement, useState } from "react"

import {
  PanelLeftDashedIcon,
  SettingsIcon,
  MessageSquareTextIcon,
  SearchIcon,
  FolderSyncIcon,
  CirclePlusIcon,
} from "lucide-react"

interface WindowProps extends ComponentProps<"div"> {
  leftToolbar?: Array<ReactElement>
  rightToolbar?: Array<ReactElement>
}

function Window({
  leftToolbar,
  rightToolbar,
  className,
  children,
  ...props
}: WindowProps) {

  const [isLeftSidebarOpen, setLeftSidebarOpen] = useState<boolean>(false)
  const [isRightSidebarOpen, setRightSidebarOpen] = useState<boolean>(false)

  return (
    <TooltipProvider delayDuration={600}>
      <div className="w-screen h-screen overflow-hidden bg-[#DCE0E8] flex flex-col">
        <Titlebar>
          <Toolbar>
            <Button variant="ghost" size="icon" tooltip="Toggle File Explorer" onClick={() => setLeftSidebarOpen(!isLeftSidebarOpen)}>
              <PanelLeftDashedIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Cloud Sync Status">
              <FolderSyncIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Command Menu">
              <SearchIcon strokeWidth={1.6} />
            </Button>
            { leftToolbar }
          </Toolbar>
          <Toolbar>
            { rightToolbar }
            <Button variant="ghost" size="icon" tooltip="New Document">
              <CirclePlusIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Agent Panel" onClick={() => setRightSidebarOpen(!isRightSidebarOpen)}>
              <MessageSquareTextIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Settings">
              <SettingsIcon strokeWidth={1.6} />
            </Button>
          </Toolbar>
        </Titlebar>
        <div className="relative h-[calc(100vh-35px)] w-full">
          <Sidebars
            isLeftOpen={isLeftSidebarOpen}
            isRightOpen={isRightSidebarOpen}
          >
            <div className={cn("bg-[#EFF1F5] outline outline-[#AEB3C0] h-full w-full", className)} {...props} >
              { children }
            </div>
          </Sidebars>
        </div>
      </div>
    </TooltipProvider>
  )
}

export {Window}
