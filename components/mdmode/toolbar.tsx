import * as React from "react"
import { Button } from "@/components/core/button"

import {
  Undo2Icon,
  Redo2Icon,
  DownloadIcon,
  NetworkIcon,
  EllipsisVerticalIcon,
} from "lucide-react"

function Toolbar() {
  return (
    <div className="flex justify-between items-center min-h-8 h-8 w-full border-[#ABB0BE] border-b px-1">
      <div>
        <Button variant="outline" size="icon" tooltip="Undo">
          <Undo2Icon strokeWidth={1.6} />
        </Button>
        <Button variant="outline" size="icon" tooltip="Redo">
          <Redo2Icon strokeWidth={1.6} />
        </Button>
      </div>
      <div>
        <Button variant="outline" size="icon" tooltip="Graph Mode">
          <NetworkIcon strokeWidth={1.6} />
        </Button>
        <Button variant="outline" size="icon">
          <EllipsisVerticalIcon strokeWidth={1.6} />
        </Button>
      </div>
    </div>
  )
}

export { Toolbar }
