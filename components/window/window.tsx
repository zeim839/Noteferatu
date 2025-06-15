import { Titlebar, Toolbar } from "./titlebar"
import { Button } from "@/components/ui/button"
import { TooltipProvider } from "@/components/ui/tooltip"
import { Sidebar } from "./sidebar"
import {
  PanelLeftDashedIcon,
  SettingsIcon,
  MessageSquareTextIcon,
  SearchIcon,
  FolderSyncIcon,
  CirclePlusIcon,
} from "lucide-react"
import {
  ComponentProps,
  createContext,
  ReactElement,
  useContext,
  useMemo,
  useState,
} from "react"

export type WindowContextProps = {
  leftToolbar?: Array<ReactElement>,
  rightToolbar?: Array<ReactElement>,
}

const WindowContext = createContext<WindowContextProps | null>(null)

function useWindow() {
  const ctx = useContext(WindowContext)
  if (!ctx) {
    throw new Error("useWindow must be used within a WindowProvider")
  }
  return ctx
}

interface WindowProviderProps extends ComponentProps<"div"> {
  leftToolbar?: Array<ReactElement>,
  rightToolbar?: Array<ReactElement>,
}

function WindowProvider({ leftToolbar, rightToolbar, className, children }: WindowProviderProps ) {
  const contextValue = useMemo<WindowContextProps>(() => ({
    leftToolbar, rightToolbar,
  }), [leftToolbar, rightToolbar])
  return (
    <WindowContext.Provider value={contextValue}>
      {children}
    </WindowContext.Provider>
  )
}

function Window({ className, children, ...props }: ComponentProps<"div">) {
  const { leftToolbar, rightToolbar } = useWindow()
  const [leftSidebarOpen, setLeftSidebarOpen] = useState<boolean>(false)
  const [rightSidebarOpen, setRightSidebarOpen] = useState<boolean>(false)
  const [leftSidebarWidth, setLeftSidebarWidth] = useState<number>(230)
  const [rightSidebarWidth, setRightSidebarWidth] = useState<number>(340)
  const [isResizing, setIsResizing] = useState<boolean>(false)

  // Calculate maximum allowed widths to prevent collision
  const maxLeftWidth = window.innerWidth - (rightSidebarOpen ? rightSidebarWidth : 0) - 100 // 100px minimum content width
  const maxRightWidth = window.innerWidth - (leftSidebarOpen ? leftSidebarWidth : 0) - 100

  const handleLeftWidthChange = (width: number) => {
    const constrainedWidth = Math.min(width, maxLeftWidth)
    setLeftSidebarWidth(constrainedWidth)
  }

  const handleRightWidthChange = (width: number) => {
    const constrainedWidth = Math.min(width, maxRightWidth)
    setRightSidebarWidth(constrainedWidth)
  }

  return (
    <TooltipProvider delayDuration={600}>
      <div className="w-screen h-screen overflow-hidden bg-[#DCE0E8] flex flex-col">
        <Titlebar>
          <Toolbar>
            <Button variant="ghost" size="icon" tooltip="Toggle File Explorer" onClick={()=>{setLeftSidebarOpen(!leftSidebarOpen)}}>
              <PanelLeftDashedIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Cloud Sync Status">
              <FolderSyncIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Command Menu">
              <SearchIcon strokeWidth={1.6} />
            </Button>
            { leftToolbar }
          </Toolbar>
          <Toolbar>
            { rightToolbar }
            <Button variant="ghost" size="icon" tooltip="New Document">
              <CirclePlusIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Agent Panel" onClick={()=>{setRightSidebarOpen(!rightSidebarOpen)}}>
              <MessageSquareTextIcon strokeWidth={1.6} />
            </Button>
            <Button variant="ghost" size="icon" tooltip="Settings">
              <SettingsIcon strokeWidth={1.6} />
            </Button>
          </Toolbar>
        </Titlebar>

        <div className="flex flex-1 relative bg-[#EFF1F5]">
          <Sidebar
            side="left"
            open={leftSidebarOpen}
            onWidthChange={handleLeftWidthChange}
            onResizeStart={() => setIsResizing(true)}
            onResizeEnd={() => setIsResizing(false)}
            defaultWidth={230}
            maxWidth={maxLeftWidth}
          />

          <Sidebar
            side="right"
            open={rightSidebarOpen}
            onWidthChange={handleRightWidthChange}
            onResizeStart={() => setIsResizing(true)}
            onResizeEnd={() => setIsResizing(false)}
            maxWidth={maxRightWidth}
            defaultWidth={340}
          />

          <div
            className={`flex-1 bg-[#EFF1F5] outline outline-[#AEB3C0] ${
              !isResizing ? 'transition-all duration-200 ease-linear' : ''
            }`}
            style={{
              marginLeft: leftSidebarOpen ? `${leftSidebarWidth}px` : '0px',
              marginRight: rightSidebarOpen ? `${rightSidebarWidth}px` : '0px'
            }}
          >
            <p className="text-black p-4">hello</p>
          </div>
        </div>
      </div>
    </TooltipProvider>
  )
}

export {WindowContext, useWindow, WindowProvider, Window}
