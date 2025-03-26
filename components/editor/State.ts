import { StateField, StateEffect } from "@codemirror/state"
import { Edge } from "@/lib/controller/EdgeController"

// Create a state field to store the current note ID
export const noteIDField = StateField.define<string | null>({
    create: () => null,
    update: (value, tr) => {
        for (const effect of tr.effects) {
            if (effect.is(setNoteIDEffect)) {
                return effect.value as string
            }
        }
        return value
    }
})
export const setNoteIDEffect = StateEffect.define<string | null>()

// Create a state field to store the list of active Edges
export const edgesField = StateField.define<Edge[] | null>({
    create: () => null,
    update: (value, tr) => {
        for (const effect of tr.effects) {
            if (effect.is(setEdgesEffect)) {
                return effect.value
            }
        }
        return value
    }
})
export const setEdgesEffect = StateEffect.define<Edge[] | null>()
