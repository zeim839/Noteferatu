import * as React from "react"

import { Toolbar } from "./toolbar"
import { useEditorContext } from "./context"

import { Node } from "@/lib/markdown"
import { Graph } from "@/components/graph/graph"

import * as Commands from "@codemirror/commands"
import * as View from "@codemirror/view"

// Editor is the markdown-mode editor.
function Editor({ node }: { node: Node }) {
  const [undoDepth, setUndoDepth] = React.useState<number>(0)
  const [redoDepth, setRedoDepth] = React.useState<number>(0)

  const viewRef = React.useRef<View.EditorView | null>(null)
  const editorRef = React.useRef(null)
  const ctx = useEditorContext()

  // Undo an editor change.
  const onUndo = () => {
    if (viewRef.current) {
      Commands.undo(viewRef.current)
    }
  }

  // Redo an editor change.
  const onRedo = () => {
    if (viewRef.current) {
      Commands.redo(viewRef.current)
    }
  }

  // Updates undo/redo history depth whenever a change is made.
  const updateHistory = () => {
    if (viewRef.current) {
      setUndoDepth(Commands.undoDepth(viewRef.current.state))
      setRedoDepth(Commands.redoDepth(viewRef.current.state))
    }
  }

  // Called when the document is updated.
  const onUpdate = (update: View.ViewUpdate) => {
    if (update.docChanged) {
      updateHistory()
    }
  }

  // Initialize codemirror editor.
  React.useEffect(() => {
    if (editorRef.current) {
      const view = new View.EditorView({
        parent: editorRef.current,
        extensions: [
          Commands.history(),
          View.EditorView.updateListener.of(onUpdate),
          View.EditorView.theme({
            "&": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
            ".cm-content": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
          }),
          View.keymap.of([...Commands.historyKeymap]),
        ],
      })
      viewRef.current = view
      return () => {
        view.destroy()
        viewRef.current = null
      }
    }
  }, [])

  return (
    <div className="flex flex-col w-full h-full bg-[#EFF1F5]">
      <Toolbar
        onUndo={onUndo}
        onRedo={onRedo}
        undoDepth={undoDepth}
        redoDepth={redoDepth}
      />
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
