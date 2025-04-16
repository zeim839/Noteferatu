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
    const view = createView('abc**bold** extra', 0)
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
