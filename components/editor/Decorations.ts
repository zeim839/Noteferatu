import { Line, RangeSetBuilder } from "@codemirror/state"

import {
  Decoration,
  DecorationSet,
  EditorView,
  ViewPlugin,
  ViewUpdate,
  WidgetType
} from "@codemirror/view"

// Range is a (from, to)-tuple type.
export type Range = {
  from : number
  to   : number
}

// DecorationContext helps decorate lines by passing editor, line
// selection, and decoration builder information to each line of the
// editor whilst parsing the document's contents.
export type DecorationContext = {
  view         : EditorView
  line         : Line
  lineRange    : Range
  selection?   : Range
  cursorOnLine : boolean
  builder      : RangeSetBuilder<Decoration>
}

// LinkWidget defines an HTML hyperlink component, which is used to
// replace markdown hyperlink expressions with HTML hyperlinks when
// parsing notes.
class LinkWidget extends WidgetType {
  constructor(readonly text: string, readonly url: string) {
    super()
  }

  toDOM() {
    const link = document.createElement("a")
    link.textContent = this.text
    link.href = this.url
    link.target = "_blank"
    link.rel = "noopener noreferrer"
    link.classList.add("cm-styled-link")
    return link
  }
}

// ImageWidget defines an HTML img component, which is used to replace
// markdown image expressions with HTML images when parsing notes.
class ImageWidget extends WidgetType {
  constructor(readonly src: string, readonly altText: string) {
    super()
  }

  toDOM() {
    const img = document.createElement("img")
    img.src = this.src
    img.alt = this.altText
    return img
  }
}

// Decorations is the markdown parser definition. It tries to match
// markdown expressions and parses them into decorations (i.e. widgets
// or styled HTML elements).
export class Decorations {
  decorations : DecorationSet

  constructor(view: EditorView) {
    this.decorations = this.createDecorations(view)
  }

  // update the editor view by calling createDecorations whenever
  // the document, viewport, or selection set changes.
  update(update : ViewUpdate) {
    this.decorations = (update.docChanged || update.selectionSet ||
      update.viewportChanged || update.focusChanged) ?
      this.createDecorations(update.view) : this.decorations
  }

  // createContext is a helper function for creating a DecorationContext
  // to be passed to a decorateX function.
  private createContext(view: EditorView, line: Line, builder: RangeSetBuilder<Decoration>): DecorationContext {
    const lineRange : Range = { from: line.from, to: line.to }
    const mainSelect = view.state.selection.main
    const selection: Range | undefined =
      view.hasFocus && mainSelect.from <= lineRange.to && mainSelect.to >= lineRange.from
        ? {
            from: Math.max(lineRange.from, mainSelect.from, mainSelect.to) - line.from,
            to: Math.min(lineRange.to, mainSelect.from, mainSelect.to) - line.from
          }
        : undefined

    const cursorOnLine = view.state.selection.ranges.some(
      range => range.from >= lineRange.from && range.from <= lineRange.to && view.hasFocus
    )

    return { view, line, lineRange, selection, cursorOnLine, builder}
  }

  // createDecorations iterates over every line and attempts to transform
  // markdown expressions into decorations.
  private createDecorations(view : EditorView) {
    const builder = new RangeSetBuilder<Decoration>()
    for (const {from, to} of view.visibleRanges) {
      let pos = from
      while (pos < to) {
        const line = view.state.doc.lineAt(pos)
        const ctx = this.createContext(view, line, builder)
        Decorations.decorateHeaders(ctx)
        Decorations.decorateBold(ctx)
        Decorations.decorateLink(ctx)
        Decorations.decorateQuote(ctx)
        Decorations.decorateImage(ctx)
        pos = line.to + 1
      }
    }
    return builder.finish()
  }

  // decorateHeaders transforms a markdown header expression into an
  // HTML header element.
  static decorateHeaders(ctx : DecorationContext) {
    const { line, selection, cursorOnLine, builder } = ctx
    const headerMatch = line.text.match(/^(#{1,6})(\s)(.*)/)
    if (headerMatch) {
      const hashLevel = headerMatch[1].length
      const hashStart = line.from + headerMatch.index!
      const spaceEnd = hashStart + headerMatch[1].length + 1
      if (!(selection || cursorOnLine)) {
        if (hashLevel == 1) {
          builder.add(
            line.from,
            line.from,
            Decoration.line({ class: "cm-line-h1" })
          )
        } else {
          builder.add(
            line.from,
            line.from,
            Decoration.line({ class: "cm-line-higher-headers" })
          )
        }

        builder.add(
          hashStart,
          spaceEnd,
          Decoration.mark({ class: "cm-hidden-characters" })
        )

        builder.add(
          spaceEnd,
          line.to,
          Decoration.mark({ class: `cm-styled-header level-${hashLevel}` })
        )
      }
      else {
        builder.add(
          hashStart,
          line.to,
          Decoration.mark({ class: `cm-styled-header level-${hashLevel}` })
        )
      }
    }
  }

  // decorateBold transforms markdown bold expressions into bold text.
  static decorateBold(ctx : DecorationContext) {
    const { line, selection, builder } = ctx
    const boldMatches = line.text.matchAll(/\*\*(.*?)\*\*/g)
    for (const boldMatch of boldMatches) {

      const start = line.from + boldMatch.index!
      const end = start + boldMatch[0].length
      const intersectingSelection = selection && selection.from < end - line.from && selection.to > start - line.from
      builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-styled-bold" })
      )

      if (!intersectingSelection) {
        // hide the asterisks
        const firstAsteriskSet = boldMatch.index! + line.from
        const secondAsteriskSet = line.from + boldMatch.index! + boldMatch[0].length - 2
        builder.add(
          firstAsteriskSet,
          firstAsteriskSet + 2,
          Decoration.mark({ class: "cm-hidden-characters" })
        )
        builder.add(
          secondAsteriskSet,
          secondAsteriskSet + 2,
          Decoration.mark({ class: "cm-hidden-characters" })
        )
      }
    }
  }

  // decorateLink transforms a markdown hyperlink into a LinkWidget.
  static decorateLink(ctx : DecorationContext) {
    const { line, selection, builder } = ctx
    const linkMatches = line.text.matchAll(/(?<!\!)\[([^\]]+)\]\(([^)]+)\)/g)
    for (const linkMatch of linkMatches) {
      const start = line.from + linkMatch.index!
      const textStart = start + 1
      const textEnd = start + linkMatch[1].length + 1
      const end = start + linkMatch[0].length
      const intersectingSelection = selection && selection.from < end - line.from && selection.to > start - line.from
      builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-styled-link" })
      )
      if (!intersectingSelection) {
        builder.add(
          start,
          start + 1,
          Decoration.mark({ class: "cm-hidden-characters" })
        )
        builder.add(
          textStart,
          textEnd,
          Decoration.widget({ widget: new LinkWidget(linkMatch[1], linkMatch[2]) })
        )
        builder.add(
          textEnd,
          end,
          Decoration.mark({ class: "cm-hidden-characters" })
        )
      }
    }
  }

  // decorateQuote transforms a markdown quote expression into a quote
  // block element.
  static decorateQuote(ctx : DecorationContext) {
    const { line, selection, cursorOnLine, builder } = ctx
    const quoteMatch = line.text.match(/>\s.+/)
    if (!quoteMatch) {
      return
    }
    const start = line.from + quoteMatch.index!
    const end = start + quoteMatch[0].length
    if (!(selection || cursorOnLine)) {
      builder.add(
        start,
        start + 1,
        Decoration.mark({ class: "cm-hidden-characters" })
      )
      builder.add(
        start + 1,
        end,
        Decoration.mark({ class: "cm-styled-quote" })
      )
    }
  }

  // decorateImage transforms a markdown embedded image expression into
  // an HTML image component (ImageWidget).
  static decorateImage(ctx : DecorationContext) {
    const { line, selection, builder } = ctx
    const imageMatch = line.text.match(/!\[(.+?)\]\((.+?)(?:\s"(.+?)")?\)(?=\s|$)/)
    if (!imageMatch) {
      return
    }
    const start = line.from + imageMatch.index!
    const end = start + imageMatch[0].length
    const intersectingSelection = selection && selection.from < end - line.from && selection.to > start - line.from
    if (!intersectingSelection) {
      builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-hidden-characters" })
      )
      builder.add(
        end,
        end,
        Decoration.widget({ widget: new ImageWidget(imageMatch[2], imageMatch[1]) })
      )
    }
  }
}

export default ViewPlugin.fromClass(Decorations,
  { decorations: (v) => v.decorations })
