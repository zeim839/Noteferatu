import * as React from "react"

import { Toolbar } from "./toolbar"
import { useEditorContext } from "./context"

import { Node } from "@/lib/markdown"
import { Root } from "@/components/markdown/root"
import { Graph } from "@/components/graph/graph"

// Editor is the markdown-mode editor.
function Editor({ node }: { node: Node }) {
  const ctx = useEditorContext()
  return (
    <div className="flex flex-col w-full bg-[#EFF1F5] max-h-[calc(100vh-35px-30px)] h-[calc(100vh-35px-30px)]">
      <Toolbar />
      <div className="flex flex-col w-full min-h-full gap-5 overflow-y-auto">
        <div data-view={ctx.view} className="data-[view=document]:block hidden w-full h-full pt-5 px-5 leading-7">
          <Root node={node} isEditable={false} />
        </div>
        {(ctx.view == 'graph') ? <Graph /> : null}
      </div>
    </div>
  )
}

export { Editor }
