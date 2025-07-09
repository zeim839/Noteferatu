"use client"

import * as React from "react"
import { Slot } from "@radix-ui/react-slot"
import { cn } from "@/lib/utils"

import { Titlebar } from "./titlebar"
import { TooltipProvider } from "@/components/core/tooltip"
import { useWindowSize } from "@/hooks/windowsize"
import { Sidebar } from "./sidebar"

type WindowContextProps = {
  isLeftSidebarOpen: boolean
  isRightSidebarOpen: boolean
  setLeftSidebarOpen: (open: boolean) => void
  setRightSidebarOpen: (open: boolean) => void

  // The initial preferred width of the left and right sidebars.
  leftDefaultWidth: number,
  rightDefaultWidth: number,
}

const WindowContext = React.createContext<WindowContextProps | null>(null)

function useWindow() {
  const ctx = React.useContext(WindowContext)
  if (!ctx) {
    throw new Error("useWindow must be used within a WindowProvider")
  }
  return ctx
}

interface WindowProviderProps extends React.ComponentProps<"div"> {
  isLeftSidebarOpen?: boolean
  isRightSidebarOpen?: boolean
  setLeftSidebarOpen?: (open: boolean) => void
  setRightSidebarOpen?: (open: boolean) => void

  leftDefaultWidth?: number,
  rightDefaultWidth?: number,
}

function WindowProvider({
  isLeftSidebarOpen,
  isRightSidebarOpen,
  setLeftSidebarOpen,
  setRightSidebarOpen,
  leftDefaultWidth = 232,
  rightDefaultWidth = 343,
  children,
  ...props
}: WindowProviderProps ) {

  const [_isLeftSidebarOpen, _setLeftSidebarOpen] = React.useState<boolean>(false)
  const [_isRightSidebarOpen, _setRightSidebarOpen] = React.useState<boolean>(false)

  // Use the parameter values when available. Otherwise use the
  // component's internal useState hooks.
  const leftOpen = isLeftSidebarOpen ?? _isLeftSidebarOpen
  const rightOpen = isRightSidebarOpen ?? _isRightSidebarOpen

  const setLeftOpen = React.useCallback(
    (value: boolean | ((value: boolean) => boolean)) => {
      const openState = typeof value === "function" ? value(_isLeftSidebarOpen) : value
      if (setLeftSidebarOpen) {
        setLeftSidebarOpen(openState)
        return
      }
      _setLeftSidebarOpen(openState)
      // TODO: MEMORIZE SIDEBAR STATE.
    },
    [setLeftSidebarOpen]
  )

  const setRightOpen = React.useCallback(
    (value: boolean | ((value: boolean) => boolean)) => {
      const openState = typeof value === "function" ? value(_isLeftSidebarOpen) : value
      if (setRightSidebarOpen) {
        setRightSidebarOpen(openState)
        return
      }
      _setRightSidebarOpen(openState)
      // TODO: MEMORIZE SIDEBAR STATE.
    },
    [setRightSidebarOpen]
  )

  // TODO: KEYBOARD SHORTCUTS.

  const contextValue = React.useMemo<WindowContextProps>(
    () => ({
      isLeftSidebarOpen: leftOpen,
      isRightSidebarOpen: rightOpen,
      setLeftSidebarOpen: setLeftOpen,
      setRightSidebarOpen: setRightOpen,
      leftDefaultWidth,
      rightDefaultWidth,
    }),
    [
      leftOpen, rightOpen, setLeftOpen,
      setRightOpen, leftDefaultWidth, rightDefaultWidth
    ]
  )

  return (
    <WindowContext.Provider value={contextValue}>
      <TooltipProvider delayDuration={600}>
        <div {...props}>
          {children}
        </div>
      </TooltipProvider>
    </WindowContext.Provider>
  )
}

// The minimum sidebar width when a sidebar is open. Has no effect when
// the sidebar is closed.
const MIN_SIDEBAR_WIDTH = 80

function Window({ className, children, ...props } : React.ComponentProps<"div">) {
  const {isLeftSidebarOpen, isRightSidebarOpen} = useWindow()
  const {leftDefaultWidth, rightDefaultWidth} = useWindow()

  // Keep track of a user configured (or default 300) preferred width.
  // When one sidebar shrinks, the other grows to its preferred width.
  // This gives resizing sidebars a sense of continuity.
  const [leftPreferredWidth, setLeftPreferredWidth] = React.useState<number>(leftDefaultWidth)
  const [rightPreferredWidth, setRightPreferredWidth] = React.useState<number>(rightDefaultWidth)
  const [leftRealWidth, setLeftRealWidth] = React.useState<number>(0)
  const [rightRealWidth, setRightRealWidth] = React.useState<number>(0)

  // Deconstruct window subcomponents.
  const slots = React.useMemo(() => {
    let leftSidebar : React.ReactNode | null = null
    let windowContent : React.ReactNode | null = null
    let rightSidebar : React.ReactNode | null = null
    let titlebar : React.ReactNode | null = null
    React.Children.forEach(children, child => {
      if (!React.isValidElement(child)) return
      if (child.type === Window.LeftSidebar) {
        leftSidebar = child
        return
      }
      if (child.type === Window.Content) {
        windowContent = child
        return
      }
      if (child.type === Window.RightSidebar) {
        rightSidebar = child
        return
      }
      if (child.type === Window.Titlebar) {
        titlebar = child
        return
      }
    })
    return { leftSidebar, windowContent, rightSidebar, titlebar }
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
    setLeftRealWidth(Math.max(Math.min(leftPreferredWidth, window.innerWidth), MIN_SIDEBAR_WIDTH))
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
    setRightRealWidth(Math.max(Math.min(rightPreferredWidth, window.innerWidth), MIN_SIDEBAR_WIDTH))
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
      <div className="w-screen h-screen bg-[#DCE0E8] flex flex-col">
        { slots.titlebar }
        <div className="relative h-[calc(100vh-35px)] h-max-[calc(100vh-35px)] w-full">
          <div className="h-full w-full" style={{ display: "grid", gridTemplateColumns: columns() }}>
            <div>
              <Sidebar
                side="left"
                open={isLeftSidebarOpen}
                onWidthChange={handleLeftWidthChange}
              >
                { slots.leftSidebar }
              </Sidebar>
            </div>
            <div className={cn("outline outline-[#ABB0BE]", className)} {...props}>
                { slots.windowContent }
            </div>
            <div>
              <Sidebar
                side="right"
                open={isRightSidebarOpen}
                onWidthChange={handleRightWidthChange}
              >
                { slots.rightSidebar }
              </Sidebar>
            </div>
          </div>
        </div>
      </div>
    </TooltipProvider>
  )
}

function LeftSidebar({ ...props }: React.ComponentProps<"div">) {
  return (<Slot {...props} />)
}

function Content({...props}: React.ComponentProps<"div">) {
  return (<Slot {...props} />)
}

function RightSidebar({...props}: React.ComponentProps<"div">) {
  return (<Slot className="h-full" {...props} />)
}

// Window subcomponents.
Window.LeftSidebar = LeftSidebar
Window.Content = Content
Window.RightSidebar = RightSidebar
Window.Titlebar = Titlebar

export { WindowProvider, Window }
