"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"

import {
  ChevronDownIcon,
  ChevronRightIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  PaperclipIcon,
  FileTextIcon,
  PlusIcon,
  Edit2Icon,
  Trash2Icon,
} from "lucide-react"

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/core/tooltip'

const tokenStr = (tokens: number) => {
  return (tokens < 1000) ? tokens.toString() :
    (tokens / 1000)
      .toFixed((tokens % 1000 == 0) ? 0 : 1)
      .toString() + 'k'
}

const timeAgo = (timestamp: number): string => {
  const currentTimeStampSeconds = Math.floor(Date.now()/1000)
  const diffInSeconds = Math.floor((currentTimeStampSeconds - timestamp))
  if (diffInSeconds < 60) return `${diffInSeconds}s ago`
  const diffInMinutes = Math.floor(diffInSeconds / 60)
  if (diffInMinutes < 60) return `${diffInMinutes}m ago`
  const diffInHours = Math.floor(diffInMinutes / 60)
  if (diffInHours < 24) return `${diffInHours}h ago`
  const diffInDays = Math.floor(diffInHours / 24)
  if (diffInDays < 30) return `${diffInDays}d ago`
  const diffInMonths = Math.floor(diffInDays / 30)
  if (diffInMonths < 12) return `${diffInMonths}mo ago`
  const diffInYears = Math.floor(diffInMonths / 12)
  return `${diffInYears}y ago`
}

type Conversation = {
  id: String
  name: String
  createdAt: number
}

const ConversationsEntry = ({ id, name, createdAt } : Conversation) => {
  const [isHovered, setIsHovered] = React.useState<boolean>(false)
  return (
    <div
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className="flex flex-row justify-between w-full p-1.5 px-2 rounded-md hover:bg-[#DCE0E8]"
    >
      <p className="text-xs text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
        {name}
      </p>
      <p
        data-is-hovered={isHovered}
        className="text-xs min-w-15 text-right data-[is-hovered=true]:hidden"
      >
        {timeAgo(createdAt)}
      </p>
      <div
        className="hidden data-[is-hovered=true]:flex gap-2 min-w-15 justify-end"
        data-is-hovered={isHovered}
      >
        <Button variant="ghost" size="icon" className="p-0 m-0 size-4">
          <Edit2Icon strokeWidth={1} className="max-h-3 max-w-3"/>
        </Button>
        <Button variant="ghost" size="icon" className="p-0 m-0 size-4">
          <Trash2Icon strokeWidth={1} className="max-h-3 max-w-3" />
        </Button>
      </div>
    </div>
  )
}

type ConversationsBodyProps = {
  visible: boolean
  conversations: Conversation[]
}

const ConversationsBody = ({ visible, conversations }: ConversationsBodyProps) => {
  if (conversations.length == 0) {
    return (
      <div className="h-full w-full flex justify-center items-center pb-16">
        <p className="text-xs">No conversations yet</p>
      </div>
    )
  }

  return (
    <div
      data-is-visible={visible}
      className="data-[is-visible=false]:hidden mt-1"
    >
      {
        conversations.map((item, index) =>
          <ConversationsEntry key={index} id={item.id} name={item.name} createdAt={item.createdAt} />
        )
      }
    </div>
  )
}

function AgentPanel() {
  const [expandConvs, setExpandConvs] = React.useState<boolean>(false)
  const [usedTokens, setUsedTokens] = React.useState<number>(0)
  const [totalTokens, setTotalTokens] = React.useState<number>(200000)
  const [conversations, setConversations] = React.useState<Conversation[]>([
    {id: "0", name: "Software Engineering Assistance Request (need help)", createdAt: 1752250136},
    {id: "1", name: "Software Engineering Help", createdAt: 1752240136},
    {id: "2", name: "Another Example Conversation", createdAt: 1752230136},
  ])

  return (
    <div className="w-full min-w-[250px] h-full flex flex-col justify-between">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1">
        <div className="flex flex-row items-center gap-1">
          <Button
            tooltip="Choose Conversation"
            onClick={ () => setExpandConvs(!expandConvs) }
            variant="ghost"
            size="icon"
          >
            {
              (expandConvs)
                ? <ChevronDownIcon strokeWidth={1.6} />
                : <ChevronRightIcon strokeWidth={1.6} />
            }
          </Button>
          <p className="text-xs max-h-[15px]">New Conversation</p>
        </div>
        <div className="flex items-center gap-1">
          <Button variant="ghost" size="icon" tooltip="New Conversation">
            <PlusIcon strokeWidth={1.6} />
          </Button>
          <Button variant="ghost" size="icon" tooltip="Agent Settings">
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>
      <div
        className="absolute top-[30px] w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={expandConvs}
      >
        <ConversationsBody
          conversations={conversations}
          visible={expandConvs}
        />
      </div>
      <div className="w-full h-[150px] bg-[#E5E9EF] outline outline-[#AEB3C0] grid grid-rows-[auto_32px]">
        <div className="bg-[#EDF0F4] p-2 flex flex-col justify-between">
          <div className="flex-1 relative">
            <textarea
              placeholder="Ask anything..."
              className="w-full h-full min-h-[60px] p-2 text-sm resize-none bg-none focus:outline-none"
            />
          </div>
          <div className="flex flex-row items-center justify-end gap-2">
            <Tooltip>
              <TooltipTrigger asChild>
                <p className="text-xs max-h-[15px]">
                  {tokenStr(usedTokens)}/{tokenStr(totalTokens)}
                </p>
              </TooltipTrigger>
              <TooltipContent>
                <p>Estimated Token Count</p>
              </TooltipContent>
            </Tooltip>
            <Button
              variant="ghost"
              size="icon"
              className="w-[20px] h-[20px]"
              tooltip="Send Message"
            >
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
