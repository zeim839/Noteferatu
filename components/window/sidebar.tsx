import { cn } from "@/lib/utils"

import * as React from "react"

interface SidebarProps extends React.ComponentProps<"div"> {
  open?: boolean
  side?: "left" | "right"
  onWidthChange?: (width: number) => void
  onResizeStart?: () => void
  onResizeEnd?: () => void
}

function Sidebar({
  open = false,
  side = "left",
  onWidthChange,
  onResizeStart,
  onResizeEnd,
  className,
  children,
  ...props
} : SidebarProps) {
  const [isResizing, setIsResizing] = React.useState<boolean>(false)

  const handleMouseDown = React.useCallback((e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)
    onResizeStart?.()
  }, [onResizeStart])

  const handleMouseMove = React.useCallback((e: MouseEvent) => {
    if (!isResizing) return
    const newWidth: number = (side === "left") ?
      e.clientX : window.innerWidth - e.clientX

    onWidthChange?.(newWidth)
  }, [isResizing, onWidthChange, side])

  const handleMouseUp = React.useCallback(() => {
    setIsResizing(false)
    onResizeEnd?.()
  }, [onResizeEnd])

  React.useEffect(() => {
    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove)
      document.addEventListener('mouseup', handleMouseUp)
      document.body.style.cursor = 'col-resize'
      document.body.style.userSelect = 'none'

      return () => {
        document.removeEventListener('mousemove', handleMouseMove)
        document.removeEventListener('mouseup', handleMouseUp)
        document.body.style.cursor = ''
        document.body.style.userSelect = ''
      }
    }
  }, [isResizing, handleMouseMove, handleMouseUp])

  return (
    <div
      data-side={side}
      data-collapsed={!open ? "true" : "false"}
      className="group hidden data-[collapsed=false]:block w-full h-full"
    >
      <div className="flex h-full flex-col relative">
        <div className={cn("h-full w-full bg-[#EFF1F5] outline outline-[#AEB3C0] overflow-hidden overscroll-none select-none cursor-default", className)} {...props}>
          {children}
        </div>
        {/* Resize handle */}
        <div
          className="absolute top-0 w-1 h-full cursor-col-resize group-data-[side=left]:right-0 group-data-[side=right]:left-0 z-40"
          onMouseDown={handleMouseDown}
        />
      </div>
    </div>
  )
}

function SidebarHeader({ className, children, ...props }:
React.ComponentProps<"div">) {
  return (
    <div
      className={cn("h-[29px] w-full min-w-max outline outline-[#ABB0BE] bg-[#DCE0E8]", className)}
      {...props}
    >
      {children}
    </div>
  )
}

Sidebar.Header = SidebarHeader

export { Sidebar, SidebarHeader }
