import * as React from "react"
import { BufferNode } from "./node"

type SplitOrientation = "vertical" | "horizontal" | null

interface BufferGroupProps {
  onClose?: () => void
}

function BufferGroup({ onClose } : BufferGroupProps) {
  const [split, setSplit] = React.useState<SplitOrientation>(null)
  const [splitPosition, setSplitPosition] = React.useState<number>(50)
  const [isDragging, setIsDragging] = React.useState<boolean>(false)
  const [isLeftClosed, setLeftClosed] = React.useState<boolean>(false)
  const [isRightClosed, setRightClosed] = React.useState<boolean>(false)
  const containerRef = React.useRef<HTMLDivElement>(null)

  const onLeftClose = () => {
    if (isRightClosed) {
      onClose?.()
    }
    setLeftClosed(true)
  }

  const onRightClose = () => {
    if (isLeftClosed) {
      onClose?.()
    }
    setRightClosed(true)
  }

  const onSplit = (orientation: SplitOrientation) => {
    setSplitPosition(50)
    setSplit(orientation)
  }

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    if (!isLeftClosed && !isRightClosed) {
      setIsDragging(true)
    }
  }

  const handleMouseMove = React.useCallback((e: MouseEvent) => {
    if (!isDragging || !containerRef.current || !split) return
    const rect = containerRef.current.getBoundingClientRect()
    let newPosition: number = Math.max(10, Math.min(
      90, (split === "vertical") ?
      ((e.clientX - rect.left) / rect.width) * 100 :
      ((e.clientY - rect.top) / rect.height) * 100
    ))
    setSplitPosition(newPosition)
  }, [isDragging, split])

  const handleMouseUp = React.useCallback(() => {
    setIsDragging(false)
  }, [])

  React.useEffect(() => {
    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove)
      document.addEventListener('mouseup', handleMouseUp)
      document.body.style.cursor = split === "vertical" ? "col-resize" : "row-resize"
      document.body.style.userSelect = "none"

      return () => {
        document.removeEventListener('mousemove', handleMouseMove)
        document.removeEventListener('mouseup', handleMouseUp)
        document.body.style.cursor = ""
        document.body.style.userSelect = ""
      }
    }
  }, [isDragging, handleMouseMove, handleMouseUp, split])

  if (split !== null) {
    const firstPanelStyle = split === "vertical"
      ? { width: `${splitPosition}%` }
      : { height: `${splitPosition}%` }

    const secondPanelStyle = split === "vertical"
      ? { width: `${100 - splitPosition}%` }
      : { height: `${100 - splitPosition}%` }

    return (
      <div
        ref={containerRef}
        data-split={split}
        data-has-closed={isLeftClosed || isRightClosed}
        className="w-full h-full flex data-[has-closed=false]:data-[split=vertical]:flex-row data-[has-closed=false]:data-[split=horizontal]:flex-col"
      >
        {
          (isLeftClosed) ? null :
            <div
              className="w-full h-full overflow-hidden"
              style={(!isRightClosed) ? firstPanelStyle : {}}>
              <BufferGroup onClose={() => onLeftClose() }/>
            </div>
        }
        {/* Resize Handle */}
        {
          (isLeftClosed || isRightClosed) ? null :
            <div
              data-split={split}
              className="bg-[#ABB0BE] data-[split=vertical]:w-px data-[split=horizontal]:h-px data-[split=vertical]:cursor-col-resize data-[split=horizontal]:cursor-row-resize"
              onMouseDown={handleMouseDown}
            />
        }
        {
          (isRightClosed) ? null :
            <div
              className="w-full h-full overflow-hidden"
              style={(!isLeftClosed) ? secondPanelStyle : {}}>
              <BufferGroup onClose={() => onRightClose() }/>
            </div>
        }
      </div>
    )
  }

  return (
    <div className="w-full h-full min-w-[400px]">
      <BufferNode onSplit={onSplit} onClose={onClose} />
    </div>
  )
}

export { BufferGroup }
