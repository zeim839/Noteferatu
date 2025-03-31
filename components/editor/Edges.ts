import { EditorView, ViewPlugin, ViewUpdate } from "@codemirror/view"
import { noteIDField, setEdgesEffect, setNoteIDEffect } from "./State"
import { Edge } from "@/lib/controller/EdgeController"
import { syntaxTree } from '@codemirror/language'
import { TreeCursor } from "@lezer/common"

export const EdgesPlugin = ViewPlugin.fromClass(
  class {
    constructor(view: EditorView) {
      this.updateEdges(view)
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.viewportChanged
         || update.transactions.some(tr => tr.effects.some(e => e.is(setNoteIDEffect))))
      {
        this.updateEdges(update.view)
      }
    }

    private updateEdges(view: EditorView) {
      const edges = this.extractEdges(view)
      setTimeout(() => {
        view.dispatch({ effects: setEdgesEffect.of(Array.from(edges)) })
      }, 0)
    }

    private extractEdges(view: EditorView): Edge[] {
      const edges: Edge[] = []
      const tree = syntaxTree(view.state)
      const cursor = tree.cursor()
      const noteID = view.state.field(noteIDField)

      do {
        if (cursor.name === "Link") {
          const edge = this.extractEdgeFromLink(cursor, view, noteID)
          if (edge) {
            edges.push(edge)
          }
        }
      } while (cursor.next())

      return edges
    }

    private extractEdgeFromLink(cursor: TreeCursor, view: EditorView, noteID: string | null): Edge | null {
      let urlStart = -1
      let urlEnd = -1
      const markers: number[] = []
      if (cursor.firstChild()) {
        do {
          if (cursor.name === "LinkMark") {
            markers.push(cursor.from)
          } else if (cursor.name === "URL") {
            urlStart = cursor.from
            urlEnd = cursor.to
          }
        } while (cursor.nextSibling())
        cursor.parent()
      }
      if (markers.length < 2 || urlStart === -1 || urlEnd === -1) {
        return null
      }

      const url = view.state.sliceDoc(urlStart, urlEnd)
      if (url.startsWith("node:") && noteID) {
        const nodeID = url.substring(5)
        if (nodeID && !isNaN(Number(nodeID))) {
          const src = Number(noteID)
          const dst = Number(nodeID)
          return { src, dst }
        }
      }
      return null
    }
  }
)
