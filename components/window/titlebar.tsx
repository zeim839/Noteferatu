import { ComponentProps } from "react"
import { cn } from "@/lib/utils"

const Titlebar = ({ children, className, ...props }: ComponentProps<"div">) => (
  <div data-tauri-drag-region className={cn("bg-[#DCE0E8] h-[35px] flex items-center", className)} {...props} >
    <div data-tauri-drag-region className="ml-[70px] px-[2px] pr-[5px] h-full w-full flex items-center justify-between z-[1000]">
      { children }
    </div>
  </div>
)

const Toolbar = ({ children, className, ...props }: ComponentProps<"div">) => (
  <div className="max-h-[35px] overflow-hidden">
    <div className={cn("flex gap-[2px] items-center", className)} {...props}>
      {children}
    </div>
  </div>
)

export {Titlebar, Toolbar}
