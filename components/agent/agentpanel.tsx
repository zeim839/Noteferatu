"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"

import {
  ChevronDownIcon,
  ZapIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  PaperclipIcon,
  FileTextIcon,
  PlusIcon,
} from "lucide-react"

function AgentPanel() {
  return (
    <div className="w-full min-w-[200px] h-full flex flex-col justify-between">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1">
        <div className="flex flex-row items-center gap-1 pl-1">
          <p className="text-xs">New Conversation</p>
          <Button variant="ghost" size="icon">
            <ChevronDownIcon strokeWidth={1.6} />
          </Button>
        </div>
        <div className="flex flex-row">
          <Button variant="ghost" size="icon" tooltip="LLM Capabilities">
            <ZapIcon strokeWidth={1.6} />
          </Button>
          <Button variant="ghost" size="icon" tooltip="Model Configuration">
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>
      <div className="w-full h-[107px] bg-[#E5E9EF] outline outline-[#AEB3C0] grid grid-rows-[auto_32px]">
        <div className="bg-[#EDF0F4] p-2 flex flex-col justify-between">
          <p className="text-sm">Ask Anything</p>
          <div className="flex flex-row justify-end">
            <Button variant="ghost" size="icon">
              <SendHorizontalIcon strokeWidth={1.6} />
            </Button>
          </div>
        </div>
        <div className="w-full bg-[#E5E9EF] outline outline-[#AEB3C0] flex flex-row">
          <div className="w-[36px] h-full outline outline-[#AEB3C0] flex items-center justify-center px-1 hover:bg-[#DCE0E8]">
            <PaperclipIcon strokeWidth={1.6} className="h-[17px]"/>
            <PlusIcon strokeWidth={1.6} className="w-[14px]" />
          </div>
          <div className="h-full w-[120px] bg-[#EDF0F4] outline outline-[#AEB3C0] flex flex-row items-center">
            <FileTextIcon strokeWidth={1.6} className="h-[14px]" />
            <p className="text-xs text-light">CIS4301_hw2.md</p>
          </div>
        </div>
      </div>
    </div>
  )
}

export { AgentPanel }
