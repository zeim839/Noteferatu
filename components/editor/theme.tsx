import * as View from "@codemirror/view"

// Defines the CSS theme for the markdown view.
const markdownTheme = View.EditorView.theme({
  "&": {
    fontFamily: "'Iosevka Comfy', monospace",
  },
  ".cm-content": {
    fontFamily: "'Iosevka Comfy', monospace",
  },
  "&.cm-focused": {
    outline: "none",
  },
  ".cm-codeblock": {
    backgroundColor: "#818b981f",
    borderRadius: "5px",
  },
  ".cm-fencedcode": {
    backgroundColor: "#818b981f",
    borderRadius: "5px",
  },
  ".cm-blockquote": {
    backgroundColor: "#818b981f",
    borderRadius: "5px",
  },
  ".cm-listitem": {
    paddingLeft: "2em",
  },
  ".cm-atxheading1": {
    fontSize: "2em",
    lineHeight: "1.25rem"
  },
  ".cm-atxheading2": {
    fontSize: "1.5em",
    lineHeight: "1.25rem"
  },
  ".cm-atxheading3": {
    fontSize: "1.25em",
  },
  ".cm-atxheading4": {
    fontSize: "1em",
  },
  ".cm-atxheading5": {
    fontSize: "0.875em",
  },
  ".cm-atxheading6": {
    fontSize: "0.85em",
    color: "#59636e"
  },
  ".cm-headermark": {
    opacity: "70%",
  },
  ".cm-emphasis": {
    fontStyle: "italic"
  },
  ".cm-strongemphasis": {
    fontWeight: "bold"
  },
  ".cm-inlinecode": {
    backgroundColor: "#818b981f",
    padding: ".2em .4em",
  },
})

export { markdownTheme }
