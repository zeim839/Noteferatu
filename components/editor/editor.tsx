import * as React from "react"

import { Toolbar } from "./toolbar"
import { useEditorContext } from "./context"

import { Node } from "@/lib/markdown"
import { Graph } from "@/components/graph/graph"

import * as Commands from "@codemirror/commands"
import * as View from "@codemirror/view"
import * as State from "@codemirror/state"

const titleDecoration = View.Decoration.mark({
  attributes: {
    class: "cm-title",
    style: "font-size: 2rem; font-weight: bold; line-height: 1.2; margin: 0.5rem 0;"
  }
})

const titleField = State.StateField.define<View.DecorationSet>({
  create() {
    return View.Decoration.none
  },
  update(decorations, tr) {
    decorations = decorations.map(tr.changes)

    // Find lines that start with "# " (markdown h1) and decorate them
    const newDecorations: Range<View.Decoration>[] = []

    for (let i = 1; i <= tr.state.doc.lines; i++) {
      const line = tr.state.doc.line(i)
      const lineText = line.text

      // Check if line starts with "# " (title marker)
      if (lineText.startsWith("# ")) {
        // Decorate from after "# " to end of line
        const from = line.from + 2
        const to = line.to
        if (from < to) {
          newDecorations.push(titleDecoration.range(from, to))
        }
      }
    }
    return View.Decoration.set(newDecorations)
  },
  provide: f => View.EditorView.decorations.from(f),
})

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
          View.EditorView.lineWrapping,
          View.EditorView.updateListener.of(onUpdate),
          titleField,
          View.EditorView.theme({
            "&": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
            ".cm-content": {
              fontFamily: "'Iosevka Comfy', monospace",
            },
            "&.cm-focused": {
              outline: "none",
            }
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
      <Toolbar onUndo={onUndo} onRedo={onRedo} undoDepth={undoDepth} redoDepth={redoDepth} />
      <div className="flex flex-col w-full h-full gap-5">
        <div data-view={ctx.view} className="data-[view=document]:block hidden pt-5 px-5 leading-7">
          <div ref={editorRef} className="w-full h-full" />
        </div>
        {(ctx.view == 'graph') ? <Graph /> : null}
      </div>
    </div>
  )
}

export { Editor }
