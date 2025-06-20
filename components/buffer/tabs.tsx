import * as React from "react"
import { Button } from "@/components/core/button"
import { XIcon } from "lucide-react"

export type TabRecord = {
  prev: TabRecord | null
  next: TabRecord | null
  name: string
  type: string
  path: string
}

type TabButtonProps = {
  index: number
  active: boolean
  name: string
  setActive: (tab: number) => void
  onCloseTab: (tab: number) => void
}

function TabButton({ index, active, name, setActive, onCloseTab }: TabButtonProps) {
  const handleClick = React.useCallback(() => {setActive(index)}, [index])
  const handleClose = React.useCallback((e: React.MouseEvent) => {
    // Prevents handleClick (which is attached to the parent) from
    // being called as well.
    e.stopPropagation()
    onCloseTab(index)
  }, [index, onCloseTab])
  return (
    <div data-tab-active={active.toString()} onClick={() => handleClick()}
      className="group relative px-3 min-w-[133px] max-w-[150px] text-sm select-none cursor-default flex items-center justify-center data-[tab-active=true]:bg-[#EFF1F5] data-[tab-active=true]:px-1 data-[tab-active=true]:border-r data-[tab-active=true]:border-[#ABB0BE] data-[tab-active=true]:border-b data-[tab-active=true]:border-b-[#EFF1F5] data-[tab-active=false]:bg-[#E5E9EF] data-[tab-active=false]:text-[#9DA0B0] data-[tab-active=false]:border-b data-[tab-active=false]:border-r data-[tab-active=false]:border-[#AEB3C0] data-[tab-active=false]:hover:bg-[#DEE2EA]"
    >
      <p className="text-nowrap text-ellipsis overflow-hidden">
        {name}
      </p>
      <Button variant="ghost" size="icon" className="hidden group-hover:flex items-center justify-center absolute right-0.5 w-4 h-4 p-0" onClick={handleClose}>
        <XIcon strokeWidth={1.6} className="max-w-[10px] max-h-[10px]" />
      </Button>
    </div>
  )
}

export { TabButton }
