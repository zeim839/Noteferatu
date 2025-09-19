import * as View from "@codemirror/view"
import * as State from "@codemirror/state"
import * as BaseParser from "@lezer/markdown"
import { Tree } from "@lezer/common"

// Defines the argument type of the plugin update function.
type UpdateData = {
  docChanged: boolean
  viewportChanged: boolean
  view: View.EditorView
}

// Container nodes.
const CodeBlock = View.Decoration.mark({ class: "cm-codeblock" })
const FencedCode = View.Decoration.mark({ class: "cm-fencedcode" })
const Blockquote = View.Decoration.mark({ class: "cm-blockquote" })
const HorizontalRule = View.Decoration.mark({ class: "cm-horizontalrule" })
const ATXHeading1 = View.Decoration.mark({ class: "cm-atxheading1" })
const ATXHeading2 = View.Decoration.mark({ class: "cm-atxheading2" })
const ATXHeading3 = View.Decoration.mark({ class: "cm-atxheading3" })
const ATXHeading4 = View.Decoration.mark({ class: "cm-atxheading4" })
const ATXHeading5 = View.Decoration.mark({ class: "cm-atxheading5" })
const ATXHeading6 = View.Decoration.mark({ class: "cm-atxheading6" })

// Leaf nodes.
const Emphasis = View.Decoration.mark({ class: "cm-emphasis" })
const StrongEmphasis = View.Decoration.mark({ class: "cm-strongemphasis" })
const InlineCode = View.Decoration.mark({ class: "cm-inlinecode" })

// Lesser tokens.
const HeaderMark = View.Decoration.mark({ class: "cm-headermark" })

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
    this.decorations = this.generate(tree)
  }

  // Process editor state changes.
  update(update: UpdateData) {
    if (update.docChanged || update.viewportChanged) {
      const tree = this.baseParser.parse(update.view.state.doc.toString())
      this.decorations = this.generate(tree)
    }
  }

  generate(tree: Tree): View.DecorationSet {
    const cursor = tree.cursor()
    let decorations: State.Range<View.Decoration>[] = []
    while (cursor.next()) {
      console.log(cursor.name)

      // Container Nodes.
      if (cursor.name === "ATXHeading1") {
        decorations.push(ATXHeading1.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading2") {
        decorations.push(ATXHeading2.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading3") {
        decorations.push(ATXHeading3.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading4") {
        decorations.push(ATXHeading4.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading5") {
        decorations.push(ATXHeading5.range(cursor.from, cursor.to))
        continue
      }
      if (cursor.name === "ATXHeading6") {
        decorations.push(ATXHeading6.range(cursor.from, cursor.to))
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
        decorations.push(HeaderMark.range(cursor.from, cursor.to+1))
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
