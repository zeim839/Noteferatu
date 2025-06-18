import * as React from "react"

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
}

function TabButton({ index, active, name, setActive }: TabButtonProps) {
  const handleClick = React.useCallback(() => {setActive(index)}, [index])
  return (
    <div data-tab-active={active.toString()} onClick={() => handleClick()}
      className="px-2 min-w-[133px] max-w-[150px] text-sm select-none cursor-default flex items-center justify-center data-[tab-active=true]:bg-[#EFF1F5] data-[tab-active=true]:px-1 data-[tab-active=true]:border-r data-[tab-active=true]:border-[#ABB0BE] data-[tab-active=true]:border-b data-[tab-active=true]:border-b-[#EFF1F5] data-[tab-active=false]:bg-[#E5E9EF] data-[tab-active=false]:text-[#9DA0B0] data-[tab-active=false]:border-b data-[tab-active=false]:border-r data-[tab-active=false]:border-[#AEB3C0] data-[tab-active=false]:hover:bg-[#DEE2EA]"
    >
      <p className="text-nowrap text-ellipsis overflow-hidden">
        {name}
      </p>
    </div>
  )
}

export { TabButton }
