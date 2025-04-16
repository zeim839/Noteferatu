import { describe, it, expect } from 'vitest'
import { EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { markdown } from '@codemirror/lang-markdown'
import { Decorations } from '../components/editor/Decorations'

function createView(doc: string, cursorPos: number = 0): EditorView {
  const state = EditorState.create({
    doc,
    selection: { anchor: cursorPos },
    extensions: [markdown()],
  })
  const view = new EditorView({ state })
  document.body.appendChild(view.dom)
  view.focus()
  return view
}

function getDecorations(view: EditorView) {
  const instance = new Decorations(view)
  const result: { from: number; to: number; class: string }[] = []

  instance.decorations.between(0, view.state.doc.length, (from, to, deco) => {
    const className = (deco.spec as any).class
    if (typeof className === 'string') {
      result.push({ from, to, class: className })
    }
  })

  return result
}

// -----------------------------
// Bold decorations
// -----------------------------
describe('Bold decorations', () => {
  it('correctly decorates **bold**', () => {
    const view = createView('**bold**')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(true)
  })

  it('correctly decorates __bold__', () => {
    const view = createView('__bold__')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(true)
  })

  it('ignores missing closing **', () => {
    const view = createView('**bold')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('ignores missing opening **', () => {
    const view = createView('bold**')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('shows markers when cursor inside bold', () => {
    const view = createView('**bold**', 4)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBe(0)
  })

  it('hides markers when cursor outside bold', () => {
    const view = createView('a**bold**', 0)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBeGreaterThanOrEqual(2)
  })

  it('handles multiple bold blocks', () => {
    const view = createView('**one** and **two**')
    const decorations = getDecorations(view)
    const count = decorations.filter((d) =>
      d.class.includes('cm-styled-bold')
    ).length
    expect(count).toBe(2)
  })

  it('ignores ** bold ** with whitespace inside', () => {
    const view = createView('** bold **')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('ignores ** bold** with leading whitespace', () => {
    const view = createView('** bold**')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('ignores **bold ** with trailing whitespace', () => {
    const view = createView('**bold **')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('ignores **  ** with only whitespace', () => {
    const view = createView('**  **')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(false)
  })

  it('handles mixed formatting like **bo*ld**', () => {
    const view = createView('**bo*ld**')
    const decorations = getDecorations(view)
    const foundBold = decorations.some((d) =>
      d.class.includes('cm-styled-bold')
    )
    expect(foundBold).toBe(true)
  })
})

// -----------------------------
// Italic decorations
// -----------------------------
describe('Italic decorations', () => {
  it('correctly decorates *italic*', () => {
    const view = createView('*italic*')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(true)
  })

  it('correctly decorates _italic_', () => {
    const view = createView('_italic_')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(true)
  })

  it('ignores missing closing *', () => {
    const view = createView('*italic')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('ignores missing opening *', () => {
    const view = createView('italic*')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('shows markers when cursor inside italic', () => {
    const view = createView('*italic*', 3)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBe(0)
  })

  it('hides markers when cursor outside italic', () => {
    const view = createView('text *italic* stuff', 0)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBeGreaterThanOrEqual(2)
  })

  it('handles multiple italic blocks', () => {
    const view = createView('*one* and *two*')
    const decorations = getDecorations(view)
    const count = decorations.filter((d) =>
      d.class.includes('cm-styled-italic')
    ).length
    expect(count).toBe(2)
  })

  it('ignores * italic * with whitespace inside', () => {
    const view = createView('* italic *')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('ignores * italic* with leading whitespace', () => {
    const view = createView('* italic*')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('ignores *italic * with trailing whitespace', () => {
    const view = createView('*italic *')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('ignores *  * with only whitespace', () => {
    const view = createView('*  *')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(false)
  })

  it('handles mixed formatting like *it_alic*', () => {
    const view = createView('*it_alic*')
    const decorations = getDecorations(view)
    const foundItalic = decorations.some((d) =>
      d.class.includes('cm-styled-italic')
    )
    expect(foundItalic).toBe(true)
  })
})

// -----------------------------
// Bold + Italic decorations
// -----------------------------
describe('Bold + Italic decorations', () => {
  it('correctly decorates ***bolditalic*** with both bold and italic styles', () => {
    const view = createView('***bolditalic***')
    const decorations = getDecorations(view)

    const bold = decorations.find((d) => d.class.includes('cm-styled-bold'))
    const italic = decorations.find((d) => d.class.includes('cm-styled-italic'))

    expect(bold).toBeTruthy()
    expect(italic).toBeTruthy()

    expect(bold!.from).toBeGreaterThanOrEqual(italic!.from)
    expect(bold!.to).toBeLessThanOrEqual(italic!.to)
  })

  it('ignores *** bold italic *** with surrounding whitespace', () => {
    const view = createView('*** bold italic ***')
    const decorations = getDecorations(view)
    const bold = decorations.find((d) => d.class.includes('cm-styled-bold'))
    const italic = decorations.find((d) => d.class.includes('cm-styled-italic'))
    expect(bold).toBeUndefined()
    expect(italic).toBeUndefined()
  })

  it('ignores ***  *** with only whitespace inside', () => {
    const view = createView('***  ***')
    const decorations = getDecorations(view)
    const bold = decorations.find((d) => d.class.includes('cm-styled-bold'))
    const italic = decorations.find((d) => d.class.includes('cm-styled-italic'))
    expect(bold).toBeUndefined()
    expect(italic).toBeUndefined()
  })

  it('shows both markers when cursor is inside ***bolditalic***', () => {
    const view = createView('***bolditalic***', 5)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBe(0)
  })

  it('hides markers when cursor is outside ***bolditalic***', () => {
    const view = createView('a***bolditalic***', 0)
    const decorations = getDecorations(view)
    const hiddenMarkers = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenMarkers.length).toBeGreaterThanOrEqual(3)
  })
})

// ---------------------------
// Header decorations
// ---------------------------
describe('Header decorations', () => {
  it('correctly decorates level 1 header', () => {
    const view = createView('# Heading 1', 0)
    const decorations = getDecorations(view)
    const lineClass = decorations.find((d) => d.class === 'cm-line-h1')
    const styledHeader = decorations.find(
      (d) => d.class === 'cm-styled-header level-1'
    )
    expect(lineClass).toBeTruthy()
    expect(styledHeader).toBeTruthy()
  })

  it('correctly decorates level 3 header', () => {
    const view = createView('### Heading 3', 0)
    const decorations = getDecorations(view)
    const lineClass = decorations.find(
      (d) => d.class === 'cm-line-higher-headers'
    )
    const styledHeader = decorations.find(
      (d) => d.class === 'cm-styled-header level-3'
    )
    expect(lineClass).toBeTruthy()
    expect(styledHeader).toBeTruthy()
  })

  it('hides header marks when cursor is outside', () => {
    const view = createView('a\n## Hidden Marks', 0)
    const decorations = getDecorations(view)
    const hidden = decorations.find((d) => d.class === 'cm-hidden-characters')
    expect(hidden).toBeTruthy()
  })

  it('shows header marks when cursor is inside', () => {
    const view = createView('## Visible Marks', 5)
    const decorations = getDecorations(view)
    const hidden = decorations.find((d) => d.class === 'cm-hidden-characters')
    expect(hidden).toBeFalsy()
  })

  it('does not decorate invalid header without space', () => {
    const view = createView('#InvalidHeader', 0)
    const decorations = getDecorations(view)
    const hasHeaderStyling = decorations.some((d) =>
      d.class.includes('cm-styled-header')
    )
    expect(hasHeaderStyling).toBe(false)
  })

  it('correctly decorates header with inline formatting', () => {
    const view = createView('## Heading with **bold** and *italic*', 0)
    const decorations = getDecorations(view)
    const header = decorations.find(
      (d) => d.class === 'cm-styled-header level-2'
    )
    const bold = decorations.find((d) => d.class === 'cm-styled-bold')
    const italic = decorations.find((d) => d.class === 'cm-styled-italic')
    expect(header).toBeTruthy()
    expect(bold).toBeTruthy()
    expect(italic).toBeTruthy()
  })
})
