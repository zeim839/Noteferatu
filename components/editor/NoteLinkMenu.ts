import { CompletionContext } from "@codemirror/autocomplete"

export const NoteLinkMenu = (context: CompletionContext) => {
    const before = context.matchBefore(/\[[^\]]*\]\(node:/)

    if (before && before.text.endsWith("(node:")) {
      return {
        from: before.from + before.text.length,
        options: [
          { label: "Option 1" },
          { label: "Option 2" },
          { label: "Option 3" } // label: title, apply: id
        ]
      }
    }

    return null
}