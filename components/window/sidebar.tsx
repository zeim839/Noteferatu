"use client"
import { ComponentProps, useState, useCallback, useEffect } from "react"

interface SidebarProps extends ComponentProps<"div"> {
  side?: "left" | "right"
  setOpen?: (open: boolean) => void
  open: boolean
  onWidthChange?: (width: number) => void
  onResizeStart?: () => void
  onResizeEnd?: () => void
  maxWidth?: number
  minWidth?: number
  defaultWidth?: number
}

function Sidebar({
  side = "left",
  open,
  setOpen,
  className,
  children,
  onWidthChange,
  onResizeStart,
  onResizeEnd,
  maxWidth,
  defaultWidth = 300,
} : SidebarProps) {
  const [width, setWidth] = useState(defaultWidth)
  const [isResizing, setIsResizing] = useState(false)
  const minWidth = 200
  const effectiveMaxWidth = maxWidth || 600

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)
    onResizeStart?.()
  }, [onResizeStart])

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!isResizing) return

    let newWidth: number
    if (side === "left") {
      newWidth = Math.min(Math.max(e.clientX, minWidth), effectiveMaxWidth)
    } else {
      // For right sidebar, calculate from right edge of screen
      newWidth = Math.min(Math.max(window.innerWidth - e.clientX, minWidth), effectiveMaxWidth)
    }

    setWidth(newWidth)
    onWidthChange?.(newWidth)
  }, [isResizing, onWidthChange, minWidth, effectiveMaxWidth, side])

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

  const sideClasses = side === "left"
    ? `left-0 ${open ? 'translate-x-0' : '-translate-x-full'}`
    : `right-0 ${open ? 'translate-x-0' : 'translate-x-full'}`

  const resizeHandleClasses = side === "left"
    ? "absolute top-0 right-0 w-1 h-full cursor-col-resize"
    : "absolute top-0 left-0 w-1 h-full cursor-col-resize"

  return (
    <div
      className={`fixed top-[35px] h-[calc(100vh-35px)] z-10 ${
        !isResizing ? 'transition-transform duration-200 ease-linear' : ''
      } ${sideClasses}`}
      style={{ width: `${width}px` }}
    >
      <div className="bg-[#E5E9EF] outline outline-[#AEB3C0] flex h-full w-full flex-col relative">
        {children}

        {/* Resize handle */}
        <div
          className={resizeHandleClasses}
          onMouseDown={handleMouseDown}
        />
      </div>
    </div>
  )
}

export { Sidebar }
