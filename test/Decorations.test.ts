import { describe, it, expect, afterEach } from 'vitest'
import { EditorState } from '@codemirror/state'
import { EditorView } from '@codemirror/view'
import { markdown } from '@codemirror/lang-markdown'
import decorationsPlugin, {
  Decorations,
} from '../components/editor/Decorations'

const activeViews: EditorView[] = []

function createView(doc: string, cursorPos: number = 0): EditorView {
  const state = EditorState.create({
    doc,
    selection: { anchor: cursorPos },
    extensions: [markdown(), decorationsPlugin],
  })
  const view = new EditorView({ state })
  document.body.appendChild(view.dom)
  view.focus()
  activeViews.push(view)
  return view
}

afterEach(() => {
  for (const view of activeViews) {
    view.destroy()
    view.dom.remove()
  }
  activeViews.length = 0
})

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

// ---------------------------
// Link decorations
// ---------------------------
describe('Link decorations', () => {
  it('decorates a basic markdown link', () => {
    const view = createView('[click here](https://example.com)', 0)
    const decorations = getDecorations(view)
    const hasLink = decorations.some((d) => d.class.includes('cm-styled-link'))
    expect(hasLink).toBe(true)
  })

  it('hides markdown characters when cursor is outside link', () => {
    const view = createView('a[click here](https://example.com)', 0)
    const decorations = getDecorations(view)
    const hidden = decorations.filter((d) => d.class === 'cm-hidden-characters')
    expect(hidden.length).toBeGreaterThanOrEqual(2)
  })

  it('shows markdown characters when cursor is inside link', () => {
    const view = createView('[click here](https://example.com)', 3)
    const decorations = getDecorations(view)
    const hidden = decorations.filter((d) => d.class === 'cm-hidden-characters')
    expect(hidden.length).toBe(0)
  })

  it('handles links with angle brackets', () => {
    const view = createView('[label](<https://example.com>)', 0)
    const decorations = getDecorations(view)
    const hasLink = decorations.some((d) => d.class.includes('cm-styled-link'))
    expect(hasLink).toBe(true)
  })

  it('ignores link with empty label', () => {
    const view = createView('[](/url)', 0)
    const decorations = getDecorations(view)
    const hasLink = decorations.some((d) => d.class.includes('cm-styled-link'))
    expect(hasLink).toBe(false)
  })

  it('handles node: links (internal)', () => {
    const view = createView('[note](node:123)', 0)
    const decorations = getDecorations(view)
    const hasLink = decorations.some((d) => d.class.includes('cm-styled-link'))
    expect(hasLink).toBe(true)
  })

  it('does not decorate text with only brackets', () => {
    const view = createView('[]()', 0)
    const decorations = getDecorations(view)
    const hasLink = decorations.some((d) => d.class.includes('cm-styled-link'))
    expect(hasLink).toBe(false)
  })

  it('renders multiple links correctly', () => {
    const view = createView('[one](https://a.com) and [two](https://b.com)', 0)
    const decorations = getDecorations(view)
    const count = decorations.filter((d) =>
      d.class.includes('cm-styled-link')
    ).length
    expect(count).toBe(2)
  })
})
describe('Link widget behavior', () => {
  it('replaces link label with a widget when cursor is outside', async () => {
    const view = createView('a[example](https://test.com)', 0)
    const linkElement = view.dom.querySelector('a')

    expect(linkElement).toBeTruthy()
    expect(linkElement?.textContent).toBe('example')
  })

  it('does not render widget for malformed link (no closing parenthesis)', () => {
    const view = createView('a[bad link](https://abc', 0)
    console.log(view.dom.innerHTML)
    const widget = view.dom.querySelector('a')
    expect(widget).toBeFalsy()
  })

  it('renders correct widget text and handles internal node: links', () => {
    const view = createView('a[note](node:456)', 0)
    const widget = view.dom.querySelector('a')
    expect(widget?.textContent).toBe('note')
  })

  it('renders widgets for multiple links', () => {
    const view = createView('a[one](https://a.com) and [two](https://b.com)', 0)
    const widgets = view.dom.querySelectorAll('a')
    expect(widgets.length).toBe(2)
    expect(widgets[0].textContent).toBe('one')
    expect(widgets[1].textContent).toBe('two')
  })
})

// ---------------------------
// Quote decorations
// ---------------------------
describe('Blockquote decorations', () => {
  it('applies quote decorations when cursor is outside the quote line', () => {
    const view = createView('> Hello world', 0)
    const decorations = getDecorations(view)
    const quoteMark = decorations.find((d) =>
      d.class.includes('cm-styled-quote')
    )
    const quoteText = decorations.find((d) =>
      d.class.includes('cm-styled-quote-text')
    )
    expect(quoteMark).toBeTruthy()
    expect(quoteText).toBeTruthy()
  })

  it('applies focused quote class when cursor is on the quote line', () => {
    const view = createView('> Hello world', 3)
    const decorations = getDecorations(view)
    const focused = decorations.find((d) =>
      d.class.includes('cm-styled-quote-focused')
    )
    expect(focused).toBeTruthy()
  })

  it('only focuses the line with the cursor in a multiline quote', () => {
    const view = createView('> Line one\n> Line two\n> Line three', 20)
    const decorations = getDecorations(view)

    const lineOne = decorations.find(
      (d) => d.from === 0 && d.class.includes('cm-styled-quote')
    )
    const lineTwo = decorations.find(
      (d) => d.from > 10 && d.class.includes('cm-styled-quote-focused')
    )
    const lineThree = decorations.find(
      (d) => d.from > 20 && d.class.includes('cm-styled-quote')
    )

    expect(lineOne).toBeTruthy()
    expect(lineTwo).toBeTruthy()
    expect(lineThree).toBeTruthy()
  })

  it('does not apply quote decorations to normal text', () => {
    const view = createView('This is just text', 0)
    const decorations = getDecorations(view)
    const hasQuote = decorations.some((d) => d.class.includes('quote'))
    expect(hasQuote).toBe(false)
  })
})

// ---------------------------
// Inline code decorations
// ---------------------------
describe('Inline code decorations', () => {
  it('correctly decorates inline code', () => {
    const view = createView('Here is `code`.', 0)
    const decorations = getDecorations(view)
    const inlineCode = decorations.find((d) =>
      d.class.includes('cm-styled-inline-code')
    )
    expect(inlineCode).toBeTruthy()
  })

  it('ignores unmatched backtick', () => {
    const view = createView('Here is `code.', 0)
    const decorations = getDecorations(view)
    const inlineCode = decorations.find((d) =>
      d.class.includes('cm-styled-inline-code')
    )
    expect(inlineCode).toBeFalsy()
  })

  it('shows backticks when cursor is inside inline code', () => {
    const view = createView('Here is `code`.', 10) // inside `code`
    const decorations = getDecorations(view)
    const hiddenTicks = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenTicks.length).toBe(0)
  })

  it('hides backticks when cursor is outside inline code', () => {
    const view = createView('Here is `code`.', 0) // outside
    const decorations = getDecorations(view)
    const hiddenTicks = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hiddenTicks.length).toBe(2)
  })

  it('handles multiple inline code blocks', () => {
    const view = createView('`one` and `two`', 0)
    const decorations = getDecorations(view)
    const inlineCodeCount = decorations.filter((d) =>
      d.class.includes('cm-styled-inline-code')
    ).length
    expect(inlineCodeCount).toBe(2)
  })
})

// ---------------------------
// Fenced code decorations
// ---------------------------
describe('Fenced code block decorations', () => {
  const codeBlock = '\n```\nconsole.log("hello")\n```'

  it('decorates code block lines with cm-styled-fenced-code', () => {
    const view = createView(codeBlock, 0)
    const decorations = getDecorations(view)
    const lineDecorations = decorations.filter((d) =>
      d.class.includes('cm-styled-fenced-code')
    )
    expect(lineDecorations.length).toBeGreaterThanOrEqual(1)
  })

  it('applies active class when cursor is inside block', () => {
    const view = createView(codeBlock, 10)
    const decorations = getDecorations(view)
    const active = decorations.find((d) =>
      d.class.includes('cm-styled-fenced-code-active')
    )
    expect(active).toBeTruthy()
  })

  it('hides backticks when cursor is outside block', () => {
    const view = createView(codeBlock, 0)
    const decorations = getDecorations(view)
    const hidden = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hidden.length).toBeGreaterThanOrEqual(2)
  })

  it('does not hide backticks when cursor is inside block', () => {
    const view = createView(codeBlock, 10)
    const decorations = getDecorations(view)
    const hidden = decorations.filter((d) =>
      d.class.includes('cm-hidden-characters')
    )
    expect(hidden.length).toBe(0)
  })

  it('handles multiple lines in block', () => {
    const view = createView('```\nline1\nline2\n```', 0)
    const decorations = getDecorations(view)
    const lines = decorations.filter((d) =>
      d.class.includes('cm-styled-fenced-code')
    )
    expect(lines.length).toBeGreaterThanOrEqual(3)
  })

  it('handles empty line inside code block', () => {
    const view = createView('```\n\nconsole.log()\n```', 0)
    const decorations = getDecorations(view)
    const lines = decorations.filter((d) =>
      d.class.includes('cm-styled-fenced-code')
    )
    expect(lines.length).toBeGreaterThanOrEqual(3)
  })

  it('ignores malformed code block (no closing ```)', () => {
    const view = createView('```\nconsole.log("missing end")', 0)
    const decorations = getDecorations(view)
    const lines = decorations.filter((d) =>
      d.class.includes('cm-styled-fenced-code')
    )
    expect(lines.length).toBeLessThan(3)
  })
})
