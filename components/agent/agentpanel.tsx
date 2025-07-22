"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { Combobox } from "@/components/core/combobox"
import * as Conversation from "./conversation"
import { AgentSettings } from "./settings"

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/core/tooltip"

import {
  ChevronDownIcon,
  ChevronRightIcon,
  ChevronsUpDownIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  PlusIcon,
} from "lucide-react"

const tokenStr = (tokens: number) => {
  return tokens < 1000
    ? tokens.toString()
    : (tokens / 1000).toFixed(tokens % 1000 == 0 ? 0 : 1).toString() + "k"
}

function AgentPanel() {
  const [expandConvs, setExpandConvs] = React.useState<boolean>(false)
  const [usedTokens /*setUsedTokens*/] = React.useState<number>(0)
  const [totalTokens /*setTotalTokens*/] = React.useState<number>(200000)
  const [settingsOpen, setSettingsOpen] = React.useState<boolean>(false)
  const [isModelSelectorOpen, setIsModelSelectorOpen] = React.useState<boolean>(false)
  const [conversations /*setConversations*/] = React.useState<Conversation.Conversation[]>([
    {
      id: "0",
      name: "Software Engineering Assistance Request (need help)",
      createdAt: 1752250136,
    },
    { id: "1", name: "Software Engineering Help", createdAt: 1752240136 },
    { id: "2", name: "Another Example Conversation", createdAt: 1752230136 },
  ])

  const onToggleConvs = () => {
    if (!expandConvs) {
      setIsModelSelectorOpen(false)
    }
    setExpandConvs(!expandConvs)
    setSettingsOpen(false)
  }

  const onToggleSettings = () => {
    if (!settingsOpen) {
      setIsModelSelectorOpen(false)
    }
    setSettingsOpen(!settingsOpen)
    setExpandConvs(false)
  }

  return (
    <div className="w-full min-w-[250px] h-full flex flex-col justify-between">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 w-full">
        <Button
          tooltip="Choose Conversation"
          onClick={onToggleConvs}
          variant="outline"
          className="p-2 rounded-md h-6 px-1 flex items-center justify-between pr-2"
        >
          {expandConvs ? (
            <ChevronDownIcon strokeWidth={1.6} />
          ) : (
            <ChevronRightIcon strokeWidth={1.6} />
          )}
          <p className="text-xs max-h-[15px] max-w-[150px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
            Conversation Title
          </p>
        </Button>
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="icon" tooltip="New Conversation">
            <PlusIcon strokeWidth={1.6} />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            tooltip="Agent Settings"
            onClick={onToggleSettings}
          >
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>

      {/* Conversation history */}
      <div
        className="absolute top-[30px] w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={expandConvs}
      >
        <Conversation.Body body={conversations} />
      </div>

      {/* Agent Settings */}
      <div
        className="absolute top-[30px] w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={settingsOpen}
      >
        <AgentSettings />
      </div>

      {/* Message Input Field */}
      <div className="w-full h-[150px] bg-[#EDF0F4] outline outline-[#AEB3C0] p-2 flex flex-col justify-between">
        <div className="flex-1 relative">
          <textarea
            placeholder="Ask anything..."
            className="w-full h-full min-h-[60px] p-2 text-sm resize-none bg-none focus:outline-none"
          />
        </div>
        <div className="flex flex-row items-center justify-between">
          <Combobox
            open={isModelSelectorOpen}
            onOpenChange={setIsModelSelectorOpen}
          >
            <Combobox.Trigger>
              <Button
                variant="outline"
                className="p-2 text-xs rounded-sm h-6 px-1 flex items-center justify-between"
              >
                No Model Selected
                <ChevronsUpDownIcon strokeWidth={1.6} />
              </Button>
            </Combobox.Trigger>
            <Combobox.Values values={[]} />
            <Combobox.EmptyBody className="p-4">
              <div className="flex flex-col items-center gap-2.5">
                <p className="text-sm text-center">
                  No models available. Please configure a LLM provider.
                </p>
                <Button size="sm" onClick={onToggleSettings}>
                  Open Agent Settings
                </Button>
              </div>
            </Combobox.EmptyBody>
          </Combobox>
          <div className="flex items-center gap-1">
            <Tooltip>
              <TooltipTrigger asChild>
                <div className="h-6 px-1 flex items-center">
                  <p className="text-xs max-h-[15px]">
                    {tokenStr(usedTokens)}/{tokenStr(totalTokens)}
                  </p>
                </div>
              </TooltipTrigger>
              <TooltipContent>
                <p>Estimated Token Count</p>
              </TooltipContent>
            </Tooltip>
            <Button variant="outline" size="icon" tooltip="Send Message">
              <SendHorizontalIcon strokeWidth={1.6} />
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}

export { AgentPanel }
