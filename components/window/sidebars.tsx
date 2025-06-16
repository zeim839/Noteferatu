import { useWindowSize } from "@/hooks/windowsize"
import { cn } from "@/lib/utils"

import {
  ComponentProps,
  useState,
  useCallback,
  useEffect,
  ReactElement,
} from "react"

// The minimum sidebar width when a sidebar is open. Has no effect when
// the sidebar is closed.
const MIN_SIDEBAR_WIDTH = 80

interface SidebarsProps extends ComponentProps<"div"> {
  isLeftOpen?: boolean
  isRightOpen?: boolean
  leftSidebarBody?: ReactElement,
  rightSidebarBody?: ReactElement,
}

function Sidebars({
  isLeftOpen = false,
  isRightOpen = false,
  leftSidebarBody,
  rightSidebarBody,
  className,
  children,
  ...props
}: SidebarsProps) {

  // Keep track of a user configured (or default 300) preferred width.
  // When one sidebar shrinks, the other grows to its preferred width.
  // This gives resizing sidebars a sense of continuity.
  const [leftPreferredWidth, setLeftPreferredWidth] = useState<number>(340)
  const [rightPreferredWidth, setRightPreferredWidth] = useState<number>(340)
  const [leftRealWidth, setLeftRealWidth] = useState<number>(0)
  const [rightRealWidth, setRightRealWidth] = useState<number>(0)

  // Handle the window size changing. The preferred widths are readjusted
  // to make things simpler (and more stable).
  useWindowSize((current, prev) => {
    const delta = current.width - prev.width
    if (delta > 0) {
      setLeftPreferredWidth(isLeftOpen ? leftRealWidth : leftPreferredWidth)
      setRightPreferredWidth(isRightOpen ? rightRealWidth : rightPreferredWidth)
      return
    }
    if (leftRealWidth + rightRealWidth >= current.width) {
      if (isLeftOpen && leftRealWidth >= rightRealWidth) {
        setLeftRealWidth(leftRealWidth + delta)
        setLeftPreferredWidth(leftRealWidth + delta)
        return
      }
      if (isRightOpen) {
        setRightRealWidth(rightRealWidth + delta)
        setRightPreferredWidth(rightRealWidth + delta)
        setLeftPreferredWidth(leftRealWidth)
      }
    }
  })

  // When the left sidebar opens, set its real width to its preferred
  // width. If it doesn't fit, set its real and preferred width to the
  // remaining available window width.
  useEffect(() => {
    if (!isLeftOpen && isRightOpen) {
      setRightRealWidth(rightPreferredWidth)
    }
    if (!isLeftOpen) {
      setLeftRealWidth(0)
      return
    }
    if (rightRealWidth >= window.innerWidth - leftPreferredWidth) {
      setRightRealWidth(Math.min(rightRealWidth, window.innerWidth - MIN_SIDEBAR_WIDTH))
      setLeftRealWidth(Math.max(window.innerWidth - rightRealWidth, MIN_SIDEBAR_WIDTH))
      setLeftPreferredWidth(Math.max(window.innerWidth - rightRealWidth, MIN_SIDEBAR_WIDTH))
      return
    }
    setLeftRealWidth(Math.min(leftPreferredWidth, window.innerWidth))
  }, [isLeftOpen])

  // Handle opening/closing the right sidebar. Same as above.
  useEffect(() => {
    if (!isRightOpen && isLeftOpen) {
      setLeftRealWidth(leftPreferredWidth)
    }
    if (!isRightOpen) {
      setRightRealWidth(0)
      return
    }
    if (leftRealWidth >= window.innerWidth - rightPreferredWidth) {
      setLeftRealWidth(Math.min(leftRealWidth, window.innerWidth - MIN_SIDEBAR_WIDTH))
      setRightRealWidth(Math.max(window.innerWidth - leftRealWidth, MIN_SIDEBAR_WIDTH))
      setRightPreferredWidth(Math.max(window.innerWidth - leftRealWidth, MIN_SIDEBAR_WIDTH))
      return
    }
    setRightRealWidth(Math.min(rightPreferredWidth, window.innerWidth))
  }, [isRightOpen])

  // Handler for when the user drags the sidebar's border to readjust
  // its width. Shrink the other sidebar's real width if user overextends.
  const handleLeftWidthChange = (width: number) => {
    if (!isLeftOpen) {
      return
    }
    if (isRightOpen && width >= window.innerWidth - rightPreferredWidth) {
      const newWidth = Math.min(width, window.innerWidth - MIN_SIDEBAR_WIDTH)
      setRightRealWidth(window.innerWidth - newWidth)
      setLeftPreferredWidth(newWidth)
      setLeftRealWidth(newWidth)
      return
    }
    const newWidth = Math.min(Math.max(width, MIN_SIDEBAR_WIDTH), window.innerWidth)
    setLeftPreferredWidth(newWidth)
    setLeftRealWidth(newWidth)
  }

  // Same as above.
  const handleRightWidthChange = (width: number) => {
    if (!isRightOpen) {
      return
    }
    if (isLeftOpen && width >= window.innerWidth - leftPreferredWidth) {
      const newWidth = Math.min(width, window.innerWidth - MIN_SIDEBAR_WIDTH)
      setLeftRealWidth(window.innerWidth - newWidth)
      setRightPreferredWidth(newWidth)
      setRightRealWidth(newWidth)
      return
    }
    const newWidth = Math.min(Math.max(width, MIN_SIDEBAR_WIDTH), window.innerWidth)
    setRightPreferredWidth(newWidth)
    setRightRealWidth(newWidth)
  }

  // Compute the CSS grid's columns based on the sidebar widths.
  const columns = () => {
    const body = `calc(100vw - ${leftRealWidth}px - ${rightRealWidth}px)`
    return `${leftRealWidth}px ${body} ${rightRealWidth}px`
  }

  return (
    <div className="h-full w-full" style={{ display: "grid", gridTemplateColumns: columns() }}>
      <div>
        <Sidebar
          side="left"
          open={isLeftOpen}
          onWidthChange={handleLeftWidthChange}
        >
          { leftSidebarBody }
        </Sidebar>
      </div>
      <div className={className} {...props}> {children} </div>
      <div>
        <Sidebar
          side="right"
          open={isRightOpen}
          onWidthChange={handleRightWidthChange}
        >
          { rightSidebarBody }
        </Sidebar>
      </div>
    </div>
  )
}

interface SidebarProps extends ComponentProps<"div"> {
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
  const [isResizing, setIsResizing] = useState<boolean>(false)

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)
    onResizeStart?.()
  }, [onResizeStart])

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!isResizing) return
    const newWidth: number = (side === "left") ?
      e.clientX : window.innerWidth - e.clientX

    onWidthChange?.(newWidth)
  }, [isResizing, onWidthChange, side])

  const handleMouseUp = useCallback(() => {
    setIsResizing(false)
    onResizeEnd?.()
  }, [onResizeEnd])

  useEffect(() => {
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
        <div className={cn("h-full w-full bg-[#E5E9EF] outline outline-[#AEB3C0]", className)} {...props}>
          {children}
        </div>
        {/* Resize handle */}
        <div
          className="absolute top-0 w-1 h-full cursor-col-resize group-data-[side=left]:right-0 group-data-[side=right]:left-0"
          onMouseDown={handleMouseDown}
        />
      </div>
    </div>
  )
}

export { Sidebar, Sidebars }
