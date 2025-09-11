import { TabRecord } from "./tabs"
import { Button } from "@/components/core/button"
import { TabButton } from "./tabs"

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger
} from "@/components/core/tooltip"

import {
  ArrowLeftIcon,
  ArrowRightIcon,
  PlusIcon,
  Columns2Icon,
} from "lucide-react"

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/core/dropdown-menu"

type HeaderProps = {
  tabs: Array<TabRecord>
  active: number
  setActive: (tab: number) => void
  onSplit: (orientation: "vertical" | "horizontal" | null) => void
  onCloseTab: (index: number) => void
  onCloseBuffer: () => void
}

function Header({ tabs, active, setActive, onSplit, onCloseTab, onCloseBuffer } : HeaderProps) {
  // The currently active tab does not have a lower border, so that it
  // looks like its merging with the buffer content. Adding an
  // outline/border to the header ruins the effect.
  return (
    <div className="h-[30px] bg-[#DCE0E8] flex flex-row">
      <div className="h-full w-[60px] flex items-center justify-between border-b border-r border-[#AEB3C0] px-1">
        <Button variant="ghost" size="icon" disabled={tabs[active].prev === null}>
          <ArrowLeftIcon strokeWidth={1.6} />
        </Button>
        <Button variant="ghost" size="icon" disabled={tabs[active].next === null}>
          <ArrowRightIcon strokeWidth={1.6} />
        </Button>
      </div>
      <div className="w-full flex flex-row overflow-x-auto scrollbar-hide">
        { /* Each tab element has its own lower & right border */ }
        {
          tabs.map((tab, i) => (
            <TabButton
              key={i}
              index={i}
              active={i === active}
              name={tab.name}
              setActive={setActive}
              onCloseTab={onCloseTab}
            />
          ))
        }
        { /* Fill the rest of the space, so the whole header can have a
        lower border. */ }
        <div className="w-full border-b border-[#AEB3C0]"/>
      </div>
      <div className="h-full w-[60px] border-b border-l border-[#AEB3C0] flex items-center">
        <Button variant="ghost" size="icon" tooltip="New Tab">
          <PlusIcon strokeWidth={1.6} />
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger>
            <Tooltip>
              <TooltipTrigger asChild>
                <div className="flex items-center justify-center rounded-md text-sm font-medium select-none w-[24px] h-[24px] hover:bg-[#D4D8E1]">
                  <Columns2Icon strokeWidth={1.6} className="w-[16px]" />
                </div>
              </TooltipTrigger>
              <TooltipContent>
                Buffer Controls
              </TooltipContent>
            </Tooltip>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuItem onClick={() => onSplit("vertical")}>
              Split Vertical
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onSplit("horizontal")}>
              Split Horizontal
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onCloseBuffer()}>
              Close Buffer
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  )
}

export { Header }
