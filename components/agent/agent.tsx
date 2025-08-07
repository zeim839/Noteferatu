"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import { Combobox } from "@/components/core/combobox"
import * as Conversation from "./conversation"
import { AgentSettings } from "./settings"
import { useAgentContext } from "./context"

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/core/tooltip"

import {
  ChevronDownIcon,
  ChevronRightIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  CrosshairIcon,
  PlusIcon,
} from "lucide-react"

// Converts a token count into a string. If less than 1000, then
// `tokens` is returned, otherwise truncate and add a 'k' prefix.
const tokenStr = (tokens: number) => {
  return tokens < 1000
    ? tokens.toString()
    : (tokens / 1000).toFixed(tokens % 1000 == 0 ? 0 : 1).toString() + "K"
}

// Agent is the parent element of the chat sidebar.
function Agent() {
  const [isModelSelectorOpen, setIsModelSelectorOpen] = React.useState<boolean>(false)
  const agentContext = useAgentContext()

  const onToggleConvs = () => {
    if (!agentContext.isConvsOpen()) {
      setIsModelSelectorOpen(false)
    }
    agentContext.toggleConvs()
  }

  const onToggleSettings = () => {
    if (!agentContext.isSettingsOpen()) {
      setIsModelSelectorOpen(false)
    }
    agentContext.toggleSettings()
  }

  const modelGroups = () => {
    const models = agentContext.models()
    const groups = []
    for (const key in models) {
      groups.push(
        <Combobox.Group key={key} heading={key}>
          {
            models[key].map((model) => (
              <Combobox.Item
                key={`${model.provider}/${model.id}`}
                value={`${model.provider}/${model.id}`}
                onSelect={() => {
                  agentContext.setSelectedModel(model)
                  setIsModelSelectorOpen(false)
                }}
              >
                {model.displayName}
              </Combobox.Item>
            ))
          }
        </Combobox.Group>
      )
    }
    return groups
  }

  return (
    <div className="w-full min-w-[250px] h-full flex flex-col justify-between">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 w-full">
        <Button
          tooltip="Choose Conversation"
          tooltipSide="bottom"
          onClick={onToggleConvs}
          variant="outline"
          className="p-2 rounded-sm h-6 px-1 flex items-center justify-between pr-2"
        >
          {agentContext.isConvsOpen() ? (
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
        data-is-expanded={agentContext.isConvsOpen()}
      >
        <Conversation.Body body={agentContext.convHistory()} />
      </div>

      {/* Agent Settings */}
      <div
        className="absolute top-[30px] w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={agentContext.isSettingsOpen()}
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
          { /*
             */}
          <div className="flex gap-1">
            <Combobox
              open={isModelSelectorOpen}
              onOpenChange={setIsModelSelectorOpen}
            >
              <Combobox.Trigger>
                <Button variant="outline" size="icon" tooltip="Select Model">
                  <CrosshairIcon strokeWidth={1.6} />
                </Button>
              </Combobox.Trigger>
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
              { modelGroups() }
            </Combobox>
            {
              (agentContext.tokensUsed() > 0) ?
                <Tooltip>
                  <TooltipTrigger asChild>
                    <div className="h-6 px-1 flex items-center">
                      <p className="text-xs max-h-[15px]">
                        {tokenStr(agentContext.tokensUsed())}
                      </p>
                    </div>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Context Usage</p>
                  </TooltipContent>
                </Tooltip> : null
            }
          </div>
          <div className="flex items-center gap-1">
            <Button variant="outline" size="icon" tooltip="Send Message">
              <SendHorizontalIcon strokeWidth={1.6} />
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}

export { Agent }
