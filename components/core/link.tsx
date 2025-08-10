import * as React from "react"
import { cn } from "@/lib/utils"
import { ExternalLinkIcon } from "lucide-react"
import { openUrl } from "@tauri-apps/plugin-opener"

interface LinkProps extends React.ComponentProps<"div"> {
  href: string
}

function Link({ href, children, className }: LinkProps) {
  return (
    <a
      onClick={() => openUrl(href)}
      className={cn(
        "inline-flex items-center w-fit gap-1 text-[#007AFF] hover:underline",
        className,
      )}
    >
      {children}
      <ExternalLinkIcon className="w-3 mr-2" />
    </a>
  )
}

export { Link }
