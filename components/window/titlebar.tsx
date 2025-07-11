import { getCurrentWindow } from "@tauri-apps/api/window"
import { ComponentProps } from "react"
import { cn } from "@/lib/utils"
import { useState, useEffect } from "react"

// Titlebar replaces the default OS titlebar.
function Titlebar({ children, className, ...props }: ComponentProps<"div">) {
  const [isFullscreen, setIsFullscreen] = useState<boolean>(false)
  useEffect(() => {
    let unlisten: (() => void) | undefined
    const spawnListener = async () => {
      const window = getCurrentWindow()
      setIsFullscreen(await window.isFullscreen())
      unlisten = await window.listen('tauri://resize', async () =>{
        setIsFullscreen(await window.isFullscreen())
      })
    }
    spawnListener()
    return () => { if (unlisten) {unlisten()} }
  }, [])
  return (
    <div data-tauri-drag-region className={cn("bg-[#DCE0E8] h-[35px] flex items-center", className)} {...props} >
      <div data-tauri-drag-region data-is-fullscreen={isFullscreen}
        className="ml-[5px] data-[is-fullscreen=false]:ml-[70px] px-[2px] pr-[5px] h-full w-full flex items-center justify-between z-[1000]"
      >
        { children }
      </div>
    </div>
  )
}

// ToolGroup contains icon buttons (tools) that appear in the TitleBar.
function ToolGroup({ children, className, ...props }:
ComponentProps<"div">) {
  return (
    <div className="max-h-[35px] overflow-hidden">
      <div className={cn("flex gap-[2px] items-center", className)} {...props}>
        {children}
      </div>
    </div>
  )
}

// Make ToolGroup a subcomponent of Titlebar.
Titlebar.ToolGroup = ToolGroup

export {Titlebar, ToolGroup}
