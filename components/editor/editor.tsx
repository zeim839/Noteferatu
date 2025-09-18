import * as React from "react"

import { Toolbar } from "./toolbar"
import { useEditorContext } from "./context"

import { Node } from "@/lib/markdown"
import { Graph } from "@/components/graph/graph"
import { EditorView } from "@codemirror/view"

// Editor is the markdown-mode editor.
function Editor({ node }: { node: Node }) {
  const ctx = useEditorContext()
  const editorRef = React.useRef(null)

  React.useEffect(() => {
    if (editorRef.current) {
      const view = new EditorView({
        doc: "",
        parent: editorRef.current,
        extensions: [
          EditorView.theme({
            "&": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
            ".cm-content": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
          }),
        ],
      })
      return () => {view.destroy()}
    }
  })

  return (
    <div className="flex flex-col w-full h-full bg-[#EFF1F5]">
      <Toolbar />
      <div className="flex flex-col w-full h-full gap-5 overflow-y-auto">
        <div data-view={ctx.view} className="data-[view=document]:block hidden pt-5 px-5 leading-7">
          <div ref={editorRef} className="w-full h-full overflow-y-auto" />
        </div>
        {(ctx.view == 'graph') ? <Graph /> : null}
      </div>
    </div>
  )
}

export { Editor }
