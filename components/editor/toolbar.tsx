import * as React from "react"

import { useEditorContext } from "./context"
import { Button } from "@/components/core/button"

import {
  Undo2Icon,
  Redo2Icon,
  NetworkIcon,
  EllipsisVerticalIcon,
} from "lucide-react"

interface ToolbarProps extends React.ComponentProps<"div"> {
  onUndo: () => void
  onRedo: () => void
  undoDepth: number
  redoDepth: number
}

function Toolbar({ onUndo, onRedo, undoDepth, redoDepth }: ToolbarProps) {
  const ctx = useEditorContext()
  const toggleView = () => {
    ctx.setView(ctx.view === 'document' ? 'graph' : 'document')
  }
  return (
    <div className="flex justify-between items-center min-h-8 h-8 w-full border-[#ABB0BE] border-b px-1">
      <div>
        <Button
          variant="outline"
          size="icon"
          tooltip="Undo"
          onClick={onUndo}
          disabled={undoDepth <= 0}
        >
          <Undo2Icon strokeWidth={1.6} />
        </Button>
        <Button
          variant="outline"
          size="icon"
          tooltip="Redo"
          onClick={onRedo}
          disabled={redoDepth <= 0}
        >
          <Redo2Icon strokeWidth={1.6} />
        </Button>
      </div>
      <div>
        <Button
          data-view={ctx.view}
          variant="outline"
          size="icon"
          tooltip="Graph Mode"
          onClick={toggleView}
          className="data-[view=graph]:bg-[#DCE0E8]"
        >
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
