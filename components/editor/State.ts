import { StateField, StateEffect } from "@codemirror/state"
import { DatabaseContextType } from "../DatabaseProvider"

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

// Create a state field to store the React only database object
export const dbField = StateField.define<DatabaseContextType | null>({
  create: () => null,
  update: (value, tr) => {
    for (const effect of tr.effects) {
      if (effect.is(setDbEffect)) {
        return effect.value
      }
    }
    return value
  }
})
export const setDbEffect = StateEffect.define<DatabaseContextType | null>()
