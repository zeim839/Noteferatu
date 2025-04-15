import { ToolImplementation } from "@/lib/OpenRouter"
import { ChatCompletionTool } from "openai/resources/index.mjs"
import { SetStateAction, Dispatch } from "react"
import { Message } from "@/lib/OpenRouter"
import { DatabaseContextType } from "../DatabaseProvider"
import UUID from "@/lib/UUID"

// ToolCall type definition
export type ToolCall = {
  tool    : 'createNote' | 'listNotes' | 'readNote' | 'searchNotes'
  id?     : number
  content : string
}

// Enhanced tool definitions with additional searchNotes function
const toolDefinitions: ChatCompletionTool[] = [
  {
    type: 'function',
    function: {
      name: 'createNote',
      description: 'Create a new note',
      parameters: {
        type: 'object',
        properties: {
          title: { type: 'string' },
          content: { type: 'string' },
        },
        required: ['title', 'content']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'listNotes',
      description: 'List all notes in the database with their titles and IDs',
      parameters: {
        type: 'object',
        properties: {},
        required: []
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'readNote',
      description: 'Read the content of a specific note by ID',
      parameters: {
        type: 'object',
        properties: {
          id: { type: 'number', description: 'The ID of the note to read' }
        },
        required: ['id']
      }
    }
  },
  {
    type: 'function',
    function: {
      name: 'searchNotes',
      description: 'Search for notes containing a specific keyword or phrase',
      parameters: {
        type: 'object',
        properties: {
          query: { type: 'string', description: 'Search term or phrase to look for in notes' }
        },
        required: ['query']
      }
    }
  }
]

// Enhanced tool implementations
const toolImplementations = (
  db : DatabaseContextType,
  setMessages: Dispatch<SetStateAction<(ToolCall | Message)[]>>
) => {
  return {
    createNote: async (args) => {
      const id = UUID()
      await db.notes.create({
        id      : id,
        title   : args.title,
        content : args.content,
        atime   : Math.floor(Date.now() / 1000),
        mtime   : Math.floor(Date.now() / 1000),
      })
      // We'll still log the creation but won't display it
      await db.history.create({
        id        : id,
        role      : 'tool',
        tool_name : 'createNote',
        content   : args.content.slice(0,200),
        time      : Math.floor(Date.now() / 1000)
      })
      return { success: true, id: id }
    },
    
    listNotes: async () => {
      // Fetch all notes from the database
      const notes = await db.notes.readAll();
      
      // Format the notes list
      const notesList = notes.map(note => ({
        id: note.id,
        title: note.title
      }));
      
      // Log the tool call but don't display it
      const toolCallId = UUID();
      await db.history.create({
        id: toolCallId,
        role: 'tool',
        tool_name: 'listNotes',
        content: JSON.stringify(notesList),
        time: Math.floor(Date.now() / 1000)
      });
      
      return { notes: notesList, success: true };
    },
    
    readNote: async (args) => {
      // Fetch the specific note from the database
      const note = await db.notes.read(args.id);
      
      if (!note) {
        return { success: false, message: "Note not found" };
      }
      
      // Format the note data
      const noteData = {
        id: note.id,
        title: note.title,
        content: note.content
      };
      
      // Log the tool call but don't display it
      const toolCallId = UUID();
      await db.history.create({
        id: toolCallId,
        role: 'tool',
        tool_name: 'readNote',
        content: JSON.stringify(noteData),
        time: Math.floor(Date.now() / 1000)
      });
      
      return { note: noteData, success: true };
    },
    
    // New search function that combines listing and reading in one step
    searchNotes: async (args) => {
      // Get all notes
      const notes = await db.notes.readAll();
      
      // Filter notes that contain the search query in title or content
      const matchedNotes = [];
      for (const note of notes) {
        // Check if query appears in title or content (case insensitive)
        if (
          note.title.toLowerCase().includes(args.query.toLowerCase()) ||
          note.content.toLowerCase().includes(args.query.toLowerCase())
        ) {
          matchedNotes.push({
            id: note.id,
            title: note.title,
            content: note.content,
            // Add snippet with context around the matched term
            snippet: extractContextSnippet(note.content, args.query)
          });
        }
      }
      
      // Log the search but don't display it
      const toolCallId = UUID();
      await db.history.create({
        id: toolCallId,
        role: 'tool',
        tool_name: 'searchNotes',
        content: JSON.stringify({ query: args.query, results: matchedNotes.length }),
        time: Math.floor(Date.now() / 1000)
      });
      
      return { 
        results: matchedNotes,
        count: matchedNotes.length,
        success: true
      };
    }
  } as ToolImplementation
}

// Helper function to extract context around matched term
function extractContextSnippet(content: string, query: string): string {
  const lowerContent = content.toLowerCase();
  const lowerQuery = query.toLowerCase();
  
  const index = lowerContent.indexOf(lowerQuery);
  if (index === -1) return "";
  
  // Get context around the match (100 chars before and after)
  const start = Math.max(0, index - 100);
  const end = Math.min(content.length, index + query.length + 100);
  
  let snippet = content.substring(start, end);
  
  // Add ellipsis if we're not at the beginning/end of content
  if (start > 0) snippet = "..." + snippet;
  if (end < content.length) snippet = snippet + "...";
  
  return snippet;
}

export { toolDefinitions, toolImplementations }