import { RangeSetBuilder } from '@codemirror/state'
import {
    Decoration,
    DecorationSet,
    EditorView,
    ViewPlugin,
    ViewUpdate,
    WidgetType,
} from '@codemirror/view'
import { syntaxTree } from '@codemirror/language'
import { TreeCursor } from '@lezer/common'

// **Link Widget**
class LinkWidget extends WidgetType {
    constructor(readonly text: string, readonly url: string) {
        super()
    }

    toDOM() {
        const link = document.createElement('a')
        link.textContent = this.text
        link.href = this.url
        link.target = '_blank'
        link.rel = 'noopener noreferrer'
        link.classList.add('cm-styled-link')
        return link
    }
}

class ImageWidget extends WidgetType {
    constructor(
        readonly src: string,
        readonly altText: string,
        private onClickReveal?: () => void
    ) {
        super()
    }

    toDOM() {
        const img = document.createElement('img')
        img.src = this.src
        img.alt = this.altText

        if (this.onClickReveal) {
            img.onclick = (event) => {
                event.preventDefault()
                event.stopPropagation()
                this.onClickReveal?.()
            }
        }

        return img
    }
}

class ConfirmImageWidget extends WidgetType {
    constructor(private onConfirm: () => void) {
        super()
    }

    toDOM(): HTMLElement {
        const button = document.createElement('button')
        button.textContent = 'Confirm Image'
        button.style.fontSize = '0.8em'
        button.style.padding = '2px 6px'
        button.style.cursor = 'pointer'
        button.style.marginLeft = '6px'
        button.style.pointerEvents = 'auto'
        button.style.zIndex = '1000'
        button.style.position = 'relative'
        button.style.border = '1px solid #ccc'
        button.style.borderRadius = '3px'
        button.style.backgroundColor = '#f9f9f9'
        button.style.color = '#333'
        button.style.fontWeight = '500'
        button.style.boxShadow = '0 1px 2px rgba(0,0,0,0.1)'

        button.onmousedown = (event) => {
            event.preventDefault()
            event.stopPropagation()
            this.onConfirm()
        }

        return button
    }
}

export class Decorations {
    decorations: DecorationSet
    imageDecorationMap: { [key: string]: { [key: string]: Decoration } } = {}

    constructor(view: EditorView) {
        this.decorations = this.createDecorations(view)
        this.imageDecorationMap = {}
    }

    update(update: ViewUpdate) {
        this.decorations =
            update.docChanged ||
            update.selectionSet ||
            update.viewportChanged ||
            update.focusChanged
                ? this.createDecorations(update.view)
                : this.decorations
    }

    private createDecorations(view: EditorView) {
        const builder = new RangeSetBuilder<Decoration>()
        const tree = syntaxTree(view.state)
        const cursor = tree.cursor()

        const decorations: {
            from: number
            to: number
            decoration: Decoration
        }[] = []

        do {
            if (cursor.name.startsWith('ATXHeading')) {
                this.decorateHeaders(cursor, decorations, view)
            } else if (cursor.name === 'StrongEmphasis') {
                this.decorateBold(cursor, decorations, view)
            } else if (cursor.name === 'Emphasis') {
                this.decorateItalic(cursor, decorations, view)
            } else if (cursor.name === 'Link') {
                this.decorateLinks(cursor, decorations, view)
            } else if (cursor.name === 'QuoteMark') {
                this.decorateQuotes(cursor, decorations, view)
            } else if (cursor.name === 'Image') {
                this.decorateImages(cursor, decorations, view)
            }
        } while (cursor.next())

        decorations.sort((a, b) => a.from - b.from)

        for (const { from, to, decoration } of decorations) {
            builder.add(from, to, decoration)
        }

        return builder.finish()
    }

    // **Decorate Headers (`# Header`)**
    private decorateHeaders(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const start = cursor.from
        const end = cursor.to
        const level = parseInt(cursor.name.replace('ATXHeading', ''), 10)
        let headerMarkEnd = start

        if (cursor.firstChild()) {
            do {
                if (cursor.name === 'HeaderMark') {
                    headerMarkEnd = cursor.to // End of `###`
                }
            } while (cursor.nextSibling())
            cursor.parent()
        }

        if (level == 1) {
            decorations.push({
                from: start,
                to: start,
                decoration: Decoration.line({ class: 'cm-line-h1' }),
            })
        } else {
            decorations.push({
                from: start,
                to: start,
                decoration: Decoration.line({
                    class: 'cm-line-higher-headers',
                }),
            })
        }

        decorations.push({
            from: start,
            to: end,
            decoration: Decoration.mark({
                class: `cm-styled-header level-${level}`,
            }),
        })

        if (!this.isCursorInside(cursor, view)) {
            decorations.push({
                from: start,
                to: headerMarkEnd + 1,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
        }
    }
    // **Decorate Bold (`**bold**`)**
    private decorateBold(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const start = cursor.from
        const end = cursor.to

        const markers: number[] = [] // Stores positions of `**` or `__`

        // Move inside `StrongEmphasis` node to find `EmphasisMark`
        if (cursor.firstChild()) {
            do {
                if (cursor.name === 'EmphasisMark') {
                    markers.push(cursor.from)
                }
            } while (cursor.nextSibling())
            cursor.parent() // Move back to the `StrongEmphasis` node
        }

        // Ensure valid bold (`**text**` → at least two `EmphasisMark`s)
        if (markers.length !== 2) {
            return // Invalid bold formatting
        }

        const cursorInside = this.isCursorInside(cursor, view)

        // **Apply bold styling**
        decorations.push({
            from: start,
            to: end,
            decoration: Decoration.mark({ class: 'cm-styled-bold' }),
        })

        // **Hide `**` or `__` markers if the cursor is NOT inside**
        if (!cursorInside) {
            decorations.push({
                from: markers[0],
                to: markers[0] + 2,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
            decorations.push({
                from: markers[1],
                to: markers[1] + 2,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
        }
    }

    // **Decorate Italic (`*italic*`)**
    private decorateItalic(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const start = cursor.from
        const end = cursor.to

        const markers: number[] = [] // Stores positions of `*` or `_`

        // Move inside `Emphasis` node to find `EmphasisMark`
        if (cursor.firstChild()) {
            do {
                if (cursor.name === 'EmphasisMark') {
                    markers.push(cursor.from)
                }
            } while (cursor.nextSibling())
            cursor.parent() // Move back to the `Emphasis` node
        }

        // Ensure valid emphasis (`*text*` → at least two `EmphasisMark`s)
        if (markers.length !== 2) {
            return // Invalid emphasis
        }

        const cursorInside = this.isCursorInside(cursor, view)

        // **Apply italics styling**
        decorations.push({
            from: start,
            to: end,
            decoration: Decoration.mark({ class: 'cm-styled-italic' }),
        })

        // **Hide `*` or `_` markers if the cursor is NOT inside**
        if (!cursorInside) {
            decorations.push({
                from: markers[0],
                to: markers[0] + 1,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
            decorations.push({
                from: markers[1],
                to: markers[1] + 1,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
        }
    }

    private decorateLinks(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const start = cursor.from
        const end = cursor.to

        let labelStart = -1,
            labelEnd = -1
        let urlStart = -1,
            urlEnd = -1
        const markers: number[] = [] // Stores positions of `LinkMark` nodes

        // Move inside the link node to find `LinkMark` and `URL`
        if (cursor.firstChild()) {
            do {
                if (cursor.name === 'LinkMark') {
                    markers.push(cursor.from) // Store positions of `LinkMark`
                } else if (cursor.name === 'URL') {
                    urlStart = cursor.from
                    urlEnd = cursor.to
                }
            } while (cursor.nextSibling())
            cursor.parent() // Move back to the link node
        }

        // Ensure we have at least `[label]`
        if (markers.length < 2) {
            return // Not a valid link
        }

        // Assign positions for label and optional URL
        labelStart = markers[0] + 1
        labelEnd = markers[1]

        // Extract link text (label)
        const label = view.state.sliceDoc(labelStart, labelEnd)

        // Extract link destination (URL)
        let url =
            urlStart !== -1 && urlEnd !== -1
                ? view.state.sliceDoc(urlStart, urlEnd)
                : ''

        // Remove enclosing `< >` for valid URIs
        if (url.startsWith('<') && url.endsWith('>')) {
            url = url.slice(1, -1)
        }

        decorations.push({
            from: start,
            to: end,
            decoration: Decoration.mark({ class: 'cm-styled-link' }),
        })

        if (!this.isCursorInside(cursor, view)) {
            // Hide `[`, `]`, `(`, `)`, but only if `()` exists
            decorations.push({
                from: markers[0],
                to: labelStart,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })
            decorations.push({
                from: labelEnd,
                to: markers[1] + 1,
                decoration: Decoration.mark({ class: 'cm-hidden-characters' }),
            })

            if (markers.length >= 4) {
                decorations.push({
                    from: markers[2],
                    to: markers[3] + 1,
                    decoration: Decoration.mark({
                        class: 'cm-hidden-characters',
                    }),
                })
            }

            if (url.trim().length > 0) {
                decorations.push({
                    from: labelStart,
                    to: labelEnd,
                    decoration: Decoration.widget({
                        widget: new LinkWidget(label, url),
                    }),
                })
            }
        }
    }
    // **Decorate Quotes (`> Quote`)**
    private decorateQuotes(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const selectedLines = new Set<number>()
        const doc = view.state.doc
        for (const range of view.state.selection.ranges) {
            const line = doc.lineAt(range.from).number
            selectedLines.add(line)
        }

        // **Process each `QuoteMark`**
        const lineNumber = doc.lineAt(cursor.from).number
        const cursorOnLine = selectedLines.has(lineNumber)

        // **Hide `>` if cursor is not on this line, but show vertical bar**
        if (!cursorOnLine) {
            decorations.push({
                from: cursor.from,
                to: cursor.from + 1,
                decoration: Decoration.mark({ class: 'cm-styled-quote' }),
            })
        } else {
            decorations.push({
                from: cursor.from,
                to: cursor.from + 1,
                decoration: Decoration.mark({
                    class: 'cm-styled-quote-focused',
                }),
            })
        }
        const lineEnd = doc.lineAt(cursor.to).to
        decorations.push({
            from: cursor.from + 1,
            to: doc.lineAt(cursor.to).to,
            decoration: Decoration.mark({ class: 'cm-styled-quote-text' }),
        })
    }

    // **Decorate Images (`![alt](url)`)**
    private decorateImages(
        cursor: TreeCursor,
        decorations: { from: number; to: number; decoration: Decoration }[],
        view: EditorView
    ) {
        const start = cursor.from
        const end = cursor.to
        let altStart = -1,
            altEnd = -1
        let urlStart = -1,
            urlEnd = -1
        const markers: number[] = []

        if (cursor.firstChild()) {
            do {
                if (cursor.name === 'LinkMark') {
                    markers.push(cursor.from)
                } else if (cursor.name === 'URL') {
                    urlStart = cursor.from
                    urlEnd = cursor.to
                }
            } while (cursor.nextSibling())
            cursor.parent()
        }

        if (markers.length < 2 || urlStart === -1 || urlEnd === -1) return

        altStart = markers[0] + 2 // After `![`
        altEnd = markers[1]
        let altText = view.state.sliceDoc(altStart, altEnd).trim()
        const url = view.state.sliceDoc(urlStart, urlEnd).trim()
        if (!url) return
        if (altText.length === 0) altText = 'Image'

        const cursorOnLine = this.isCursorInside(cursor, view)

        if (!cursorOnLine) {
            const imageDecoration = this.imageDecorationMap[url]?.[altText]
            if (!imageDecoration) {
                if (!this.imageDecorationMap[url])
                    this.imageDecorationMap[url] = {}
                this.imageDecorationMap[url][altText] = Decoration.widget({
                    widget: new ImageWidget(url, altText, () => {
                        const line = view.state.doc.lineAt(altStart)
                        view.dispatch({
                            selection: { anchor: line.from, head: line.to }, // highlights the whole line
                            scrollIntoView: true,
                        })
                    }),
                })
            }
            decorations.push({
                from: start,
                to: end,
                decoration: Decoration.mark({
                    class: 'cm-hide-image-line',
                }),
            })
            decorations.push({
                from: end,
                to: end,
                decoration: this.imageDecorationMap[url][altText],
            })
        }
    }

    private isCursorInside(cursor: TreeCursor, view: EditorView) {
        const cursorPos = view.state.selection.main.head
        const selection = view.state.selection.main
        const cursorInside = cursorPos >= cursor.from && cursorPos <= cursor.to
        const selectionInside =
            selection.from <= cursor.to && selection.to >= cursor.from
        return cursorInside || selectionInside
    }
}

export default ViewPlugin.fromClass(Decorations, {
    decorations: (v) => v.decorations,
})
