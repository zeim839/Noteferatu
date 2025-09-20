import * as View from "@codemirror/view"
import * as State from "@codemirror/state"
import * as BaseParser from "@lezer/markdown"
import { Tree } from "@lezer/common"

// Defines the argument type of the plugin update function.
type UpdateData = {
  docChanged: boolean
  viewportChanged: boolean
  view: View.EditorView
  selectionSet: boolean
}

// Container nodes.
const CodeBlock = View.Decoration.line({ class: "cm-codeblock" })
const FencedCode = View.Decoration.line({ class: "cm-fencedcode" })
const Blockquote = View.Decoration.line({ class: "cm-blockquote" })
const HorizontalRule = View.Decoration.mark({ class: "cm-horizontalrule" })

const BulletList = View.Decoration.line({ class: "cm-bulletlist" })
const OrderedList = View.Decoration.line({ class: "cm-orderedlist" })
const ListItem = View.Decoration.mark({ class: "cm-listitem" })

const ATXHeading1 = View.Decoration.line({ class: "cm-atxheading1" })
const ATXHeading2 = View.Decoration.line({ class: "cm-atxheading2" })
const ATXHeading3 = View.Decoration.line({ class: "cm-atxheading3" })
const ATXHeading4 = View.Decoration.line({ class: "cm-atxheading4" })
const ATXHeading5 = View.Decoration.line({ class: "cm-atxheading5" })
const ATXHeading6 = View.Decoration.line({ class: "cm-atxheading6" })

// Leaf nodes.
const Emphasis = View.Decoration.mark({ class: "cm-emphasis" })
const StrongEmphasis = View.Decoration.mark({ class: "cm-strongemphasis" })
const InlineCode = View.Decoration.mark({ class: "cm-inlinecode" })

// Lesser tokens.
const HeaderMark = View.Decoration.mark({ class: "cm-headermark" })
const ListMark = View.Decoration.mark({ class: "cm-listmark" })

// Generator transforms plain text into rich codemirror widgets.
class Generator {

  // Accumulates decorations to pass to the editor.
  decorations: View.DecorationSet

  // The Lezer markdown parser instance.
  baseParser: BaseParser.MarkdownParser

  // Initialize the decoration set and baseParser.
  constructor(view: View.EditorView) {
    this.baseParser = BaseParser.parser.configure(BaseParser.GFM)
    const tree = this.baseParser.parse(view.state.doc.toString())
    this.decorations = this.generate(view, tree)
  }

  // Process editor state changes.
  update(update: UpdateData) {
    if (update.docChanged || update.viewportChanged) {
      const tree = this.baseParser.parse(update.view.state.doc.toString())
      this.decorations = this.generate(update.view, tree)
    }
  }

  generate(view: View.EditorView, tree: Tree): View.DecorationSet {
    const cursor = tree.cursor()
    let decorations: State.Range<View.Decoration>[] = []
    const doc = view.state.doc

    while (cursor.next()) {
      console.log(cursor.name)

      // Container Nodes.
      if (cursor.name === "CodeBlock") {
        const startLine = doc.lineAt(cursor.from)
        const endLine = doc.lineAt(cursor.to)

        // Add line decorations
        // TODO: First and last lines should be different.
        // (i.e. fix border radius and top/bottom padding).
        for (let lineNum = startLine.number; lineNum <= endLine.number; lineNum++) {
          const line = doc.line(lineNum)
          decorations.push(CodeBlock.range(line.from))
        }

        continue
      }
      if (cursor.name === "FencedCode") {
        const startLine = doc.lineAt(cursor.from)
        const endLine = doc.lineAt(cursor.to)

        // Add line decorations
        // TODO: First and last lines should be different.
        // (i.e. fix border radius and top/bottom padding).
        for (let lineNum = startLine.number; lineNum <= endLine.number; lineNum++) {
          const line = doc.line(lineNum)
          decorations.push(FencedCode.range(line.from))
        }

        continue
      }
      if (cursor.name === "Blockquote") {
        const startLine = doc.lineAt(cursor.from)
        const endLine = doc.lineAt(cursor.to)

        // Add line decorations
        // TODO: First and last lines should be different.
        // (i.e. fix border radius and top/bottom padding).
        for (let lineNum = startLine.number; lineNum <= endLine.number; lineNum++) {
          const line = doc.line(lineNum)
          decorations.push(Blockquote.range(line.from))
        }

        continue
      }
      if (cursor.name === "ListItem") {
        decorations.push(ListItem.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading1") {
        decorations.push(ATXHeading1.range(cursor.from))
        continue
      }
      if (cursor.name === "ATXHeading2") {
        decorations.push(ATXHeading2.range(cursor.from))
        continue
      }
      if (cursor.name === "ATXHeading3") {
        decorations.push(ATXHeading3.range(cursor.from))
        continue
      }
      if (cursor.name === "ATXHeading4") {
        decorations.push(ATXHeading4.range(cursor.from))
        continue
      }
      if (cursor.name === "ATXHeading5") {
        decorations.push(ATXHeading5.range(cursor.from))
        continue
      }
      if (cursor.name === "ATXHeading6") {
        decorations.push(ATXHeading6.range(cursor.from))
        continue
      }

      // Leaf Nodes.
      if (cursor.name === "InlineCode") {
        decorations.push(InlineCode.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "Emphasis") {
        decorations.push(Emphasis.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "StrongEmphasis") {
        decorations.push(StrongEmphasis.range(cursor.from, cursor.to))
        continue
      }

      // Lesser Tokens.
      if (cursor.name === "HeaderMark") {
        decorations.push(HeaderMark.range(cursor.from, cursor.to))
        continue
      }

      // Handle list markers (bullets, numbers, etc.)
      if (cursor.name === "ListMark") {
        decorations.push(ListMark.range(cursor.from, cursor.to))
        continue
      }
    }

    return View.Decoration.set(decorations)
  }
}

const markdownPlugin = View.ViewPlugin.fromClass(Generator, {
  decorations: (v) => v.decorations,
})

export { markdownPlugin }
