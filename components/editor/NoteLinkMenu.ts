import { CompletionContext } from "@codemirror/autocomplete"
import { Note } from "@/lib/controller/NoteController"

export const NoteLinkMenu = (notes: Note[]) => (context: CompletionContext) => {
    const before = context.matchBefore(/\[[^\]]*\]\(node:([^)]*)/)
    if (!before) return null

    const query = before.text.match(/\[[^\]]*\]\(node:(.*)$/)?.[1] || ""
    const from = before.from + before.text.length - query.length
    const sortedNotes = [...notes].sort((a, b) => b.mtime - a.mtime)
    let filteredNotes: Note[]
    if (query.length === 0) {
        filteredNotes = sortedNotes.slice(0, 3)
    } else {
        filteredNotes = sortedNotes.filter(note =>
          note.title.toLowerCase().startsWith(query.toLowerCase())
        )
    }

    if (filteredNotes.length === 0) {
        return {
          from,
          options: [{ label: "No Results Found", apply: "-1" }],
          filter: false
        }
    }
    const options = filteredNotes.map(note => ({
      label: note.title,
      apply: `${note.id}`
    }))
    return {
      from,
      options,
      filter: false
    }
}