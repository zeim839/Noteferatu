import { Button } from "@/components/core/button"
import { cn } from "@/lib/utils"

import {
  ArrowLeftIcon,
  ArrowRightIcon,
  PlusIcon,
  Columns2Icon,
} from "lucide-react"

function Tab({ selected=false }: {selected?: boolean}) {
  return (
    <div className={cn("w-[133px] flex items-center justify-center text-sm", (selected) ? "bg-[#EFF1F5] h-[31px] border-l border-r border-[#ABB0BE] border-b border-b-[#EFF1F5] mb-[-1px] relative -mx-[1px]" : "bg-[#E5E9EF] h-full text-[#9DA0B0] outline outline-[#ABB0BE]")}>
      <p>Introduction</p>
    </div>
  )
}

function TabMenu() {
  return (
    <div className="w-full h-[30px] bg-[#DCE0E8] outline outline-[#ABB0BE] grid grid-cols-[60px_auto_60px]">
      <div className="w-[60px] h-full outline outline-[#AEB3C0] flex items-center justify-between px-1.5">
        <Button variant="ghost" size="icon">
          <ArrowLeftIcon strokeWidth={1.6} className="size-3.5" />
        </Button>
        <Button variant="ghost" size="icon">
          <ArrowRightIcon strokeWidth={1.6} className="size-3.5" />
        </Button>
      </div>
      <div className="overflow-visible flex flex-row">
        <Tab selected={true} />
        <Tab />
      </div>
      <div className="h-full outline outline-[#AEB3C0] flex items-center justify-between px-1.5">
        <Button variant="ghost" size="icon" tooltip="New Tab">
          <PlusIcon strokeWidth={1.6} />
        </Button>
        <Button variant="ghost" size="icon" tooltip="Split Window">
          <Columns2Icon strokeWidth={1.6} />
        </Button>
      </div>
    </div>
  )
}

function VBuffer() {
  return (
    <div className="grid grid-rows-[30px_auto] w-full h-full">
      <div>
        <TabMenu />
      </div>
      <div className="w-full h-full bg-[#EFF1F5] outline outline-[#AEB3C0]" />
    </div>
  )
}

export { VBuffer, TabMenu }
