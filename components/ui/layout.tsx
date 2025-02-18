// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
"use client";

import { Button } from "@/components/ui/button";
import { AlignJustify, MessageSquare, Settings } from "lucide-react";
import { ReactNode, useContext, createContext, useState } from "react";
import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandItem,
} from "@/components/ui/command";

// Handles layout state.
type LayoutContextType = {
  setRecentsOpen: (open: boolean) => void;
  setChatOpen: (open: boolean) => void;
  isRecentsOpen: boolean;
  isChatOpen: boolean;
  notes: Note[];
  searchResults: Note[];
  setSearchResults: (results: Note[]) => void;
};

type Note = {
  id: string;
  title: string;
  content: string;
};

// Handles navigation layout state context.
const LayoutContext = createContext<LayoutContextType | null>(null);

// Exposes layout context data within a LayoutProvider.
const useLayout = () => {
  const context = useContext(LayoutContext);
  if (!context) {
    throw new Error("useLayout must be used within a LayoutProvider");
  }
  return context;
};

// LayoutProvider handles its own LayoutContext and exposes it to children via a React context provider.
const LayoutProvider = ({ children }: { children: React.ReactNode }) => {
  const [isRecentsOpen, setRecentsOpen] = useState(false);
  const [isChatOpen, setChatOpen] = useState(false);

  // State for notes and search results
  const [notes, setNotes] = useState<Note[]>([
    { id: "1", title: "First Note", content: "This is the first note content." },
    { id: "2", title: "Second Note", content: "This is the second note content." },
    { id: "3", title: "Third Note", content: "Another note with unique content." },
  ]);
  const [searchResults, setSearchResults] = useState<Note[]>([]);

  return (
    <LayoutContext.Provider
      value={{
        isRecentsOpen,
        setRecentsOpen,
        isChatOpen,
        setChatOpen,
        notes,
        searchResults,
        setSearchResults,
      }}
    >
      {children}
    </LayoutContext.Provider>
  );
};

const Layout = () => {
  const { notes, searchResults, setSearchResults } = useLayout();
  console.log("mounted");

  // Filter notes based on the search query
  const handleSearch = (query: string) => {
    console.log("Typing in search:", query); // This should show typing input
    const filteredNotes = notes.filter(
      (note) =>
        note.title.toLowerCase().includes(query.toLowerCase()) ||
        note.content.toLowerCase().includes(query.toLowerCase())
    );
    console.log("Filtered notes:", filteredNotes); // Debugging: check filtered results
    setSearchResults(filteredNotes);
  };

  // Handle pressing the Enter key
  const handleEnterKey = (query: string) => {
    console.log("Search submitted:", query); // Logs the search query when Enter is pressed
    handleSearch(query); // Ensure search happens on Enter as well
  };

  return (
    <div className="fixed z-100 w-full flex flex-row p-3 justify-between">
      <div className="flex flex-row gap-2">
        <Command>
          <CommandInput
            placeholder="Search Notes"
            onChange={(e) => handleSearch(e.target.value)} // Forwarding onChange to handle search
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleEnterKey(e.currentTarget.value); // Handle Enter key press
                handleSearch(e.currentTarget.value); // Ensure search happens on Enter as well
              }
            }}
          />
          <CommandList>
            {searchResults.length > 0 ? (
              searchResults.map((note) => (
                <CommandItem key={note.id}>
                  {note.title}
                </CommandItem>
              ))
            ) : (
              <CommandEmpty>No results found.</CommandEmpty>
            )}
          </CommandList>
        </Command>
      </div>
    </div>
  );
};


// LayoutContent handles sidebar and main content.
const LayoutContent = ({ children }: { children?: ReactNode }) => {
  const { isRecentsOpen, isChatOpen } = useLayout();
  return (
    <div className="flex justify-between">
      {isRecentsOpen && <div className="w-[450px] h-screen bg-[#f5f5f5cc]" />}
      <div className="w-full h-full pt-16">{children}</div>
      {isChatOpen && <div className="w-[370px] h-screen bg-[#f5f5f5cc]" />}
    </div>
  );
};

export { LayoutProvider, LayoutContext, Layout, LayoutContent, useLayout };
