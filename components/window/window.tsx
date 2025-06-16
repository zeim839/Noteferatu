"use client"

import * as React from "react"

import { Titlebar, Toolbar } from "./titlebar"
import { Button } from "@/components/ui/button"
import { TooltipProvider } from "@/components/ui/tooltip"
import { useWindowSize } from "@/hooks/windowsize"
import { Sidebar } from "./sidebar"

import {
  PanelLeftDashedIcon,
  SettingsIcon,
  MessageSquareTextIcon,
  SearchIcon,
  FolderSyncIcon,
  CirclePlusIcon,
} from "lucide-react"

// The minimum sidebar width when a sidebar is open. Has no effect when
// the sidebar is closed.
const MIN_SIDEBAR_WIDTH = 80

interface WindowProps extends React.ComponentProps<"div"> {
}

function Window({ className, children, ...props } : WindowProps) {
  const [isLeftSidebarOpen, setLeftSidebarOpen] = React.useState<boolean>(false)
  const [isRightSidebarOpen, setRightSidebarOpen] = React.useState<boolean>(false)

  // Keep track of a user configured (or default 300) preferred width.
  // When one sidebar shrinks, the other grows to its preferred width.
  // This gives resizing sidebars a sense of continuity.
  const [leftPreferredWidth, setLeftPreferredWidth] = React.useState<number>(340)
  const [rightPreferredWidth, setRightPreferredWidth] = React.useState<number>(340)
  const [leftRealWidth, setLeftRealWidth] = React.useState<number>(0)
  const [rightRealWidth, setRightRealWidth] = React.useState<number>(0)

  // Deconstruct window subcomponents.
  const slots = React.useMemo(() => {
    let leftSidebarBody : React.ReactNode | null = null
    let windowContent : React.ReactElement | null = null
    let rightSidebarBody : React.ReactElement | null = null
    React.Children.forEach(children, child => {
      if (!React.isValidElement(child)) return
      if (child.type === Window.LeftSidebarBody) {
        leftSidebarBody = child
        return
      }
      if (child.type === Window.Content) {
        windowContent = child
        return
      }
      if (child.type === Window.RightSidebarBody) {
        rightSidebarBody = child
        return
      }
    })
    return { leftSidebarBody, windowContent, rightSidebarBody }
  }, [children])

  // Handle the window size changing. The preferred widths are readjusted
  // to make things simpler (and more stable).
  useWindowSize((current, prev) => {
    const delta = current.width - prev.width
    if (delta > 0) {
      setLeftPreferredWidth(isLeftSidebarOpen ? leftRealWidth : leftPreferredWidth)
      setRightPreferredWidth(isRightSidebarOpen ? rightRealWidth : rightPreferredWidth)
      return
    }
    if (leftRealWidth + rightRealWidth >= current.width) {
      if (isLeftSidebarOpen && leftRealWidth >= rightRealWidth) {
        setLeftRealWidth(leftRealWidth + delta)
        setLeftPreferredWidth(leftRealWidth + delta)
        return
      }
      if (isRightSidebarOpen) {
        setRightRealWidth(rightRealWidth + delta)
        setRightPreferredWidth(rightRealWidth + delta)
        setLeftPreferredWidth(leftRealWidth)
      }
    }
  })

  // When the left sidebar opens, set its real width to its preferred
  // width. If it doesn't fit, set its real and preferred width to the
  // remaining available window width.
  React.useEffect(() => {
    if (!isLeftSidebarOpen && isRightSidebarOpen) {
      setRightRealWidth(rightPreferredWidth)
    }
    if (!isLeftSidebarOpen) {
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
  }, [isLeftSidebarOpen])

  // Handle opening/closing the right sidebar. Same as above.
  React.useEffect(() => {
    if (!isRightSidebarOpen && isLeftSidebarOpen) {
      setLeftRealWidth(leftPreferredWidth)
    }
    if (!isRightSidebarOpen) {
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
  }, [isRightSidebarOpen])

    // Handler for when the user drags the sidebar's border to readjust
  // its width. Shrink the other sidebar's real width if user overextends.
  const handleLeftWidthChange = (width: number) => {
    if (!isLeftSidebarOpen) {
      return
    }
    if (isRightSidebarOpen && width >= window.innerWidth - rightPreferredWidth) {
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
    if (!isRightSidebarOpen) {
      return
    }
    if (isLeftSidebarOpen && width >= window.innerWidth - leftPreferredWidth) {
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
    <TooltipProvider delayDuration={600}>
      <div className="w-screen h-screen overflow-hidden bg-[#DCE0E8] flex flex-col">
        <Titlebar>
          <Toolbar>
            <Button variant="ghost" size="icon" tooltip="Toggle File Explorer" onClick={() => setLeftSidebarOpen(!isLeftSidebarOpen)}>
              <PanelLeftDashedIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Cloud Sync Status">
              <FolderSyncIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Command Menu">
              <SearchIcon strokeWidth={1.6} />
            </Button>
          </Toolbar>
          <Toolbar>
            <Button variant="ghost" size="icon" tooltip="New Document">
              <CirclePlusIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Agent Panel" onClick={() => setRightSidebarOpen(!isRightSidebarOpen)}>
              <MessageSquareTextIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Settings">
              <SettingsIcon strokeWidth={1.6} />
            </Button>
          </Toolbar>
        </Titlebar>
        <div className="relative h-[calc(100vh-35px)] w-full">
          <div className="h-full w-full" style={{ display: "grid", gridTemplateColumns: columns() }}>
            <div>
              <Sidebar
                side="left"
                open={isLeftSidebarOpen}
                onWidthChange={handleLeftWidthChange}
              >
                { slots.leftSidebarBody }
              </Sidebar>
            </div>
            <div className={className} {...props}>
              { slots.windowContent }
            </div>
            <div>
              <Sidebar
                side="right"
                open={isRightSidebarOpen}
                onWidthChange={handleRightWidthChange}
              >
                { slots.rightSidebarBody }
              </Sidebar>
            </div>
          </div>
        </div>
      </div>
    </TooltipProvider>
  )
}

Window.LeftSidebarBody = ({ ...props } : React.ComponentProps<"div">) => (
  <div {...props} />
)

Window.Content = ({ ...props } : React.ComponentProps<"div">) => (
  <div {...props} />
)

Window.RightSidebarBody = ({ ...props } : React.ComponentProps<"div">) => (
  <div {...props} />
)

export { Window }
