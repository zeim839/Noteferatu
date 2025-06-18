import * as React from "react"
import { BufferNode } from "./node"

type SplitOrientation = "vertical" | "horizontal" | null

function BufferGroup() {
  const [split, setSplit] = React.useState<SplitOrientation>(null)
  const onSplit = (orientation: SplitOrientation) => setSplit(orientation)
  if (split !== null) {
    return (
      <div
        data-split={split}
        className="h-full grid data-[split=vertical]:grid-cols-2 data-[split=horizontal]:grid-rows-2"
      >
        <div className="outline outline-[#AEB3C0] overflow-hidden">
          <BufferNode onSplit={onSplit} />
        </div>
        <div className="outline outline-[#AEB3C0] overflow-hidden">
          <BufferNode onSplit={onSplit} />
        </div>
      </div>
    )
  }

  return (
    <div className="w-full h-full min-w-[400px]">
      <BufferNode onSplit={onSplit} />
    </div>
  )
}

export { BufferGroup }
