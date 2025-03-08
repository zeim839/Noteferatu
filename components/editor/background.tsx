"use client"

import { createContext, useContext, useState, ReactNode } from 'react'

type EditorBackgroundContextType = {
  setEditorMode: (active: boolean) => void;
  isEditorMode: boolean;
}

const EditorBackgroundContext = createContext<EditorBackgroundContextType>({
  setEditorMode: () => {},
  isEditorMode: false
});

export const EditorBackgroundProvider = ({ children }: {children: ReactNode}) => {
  const [isEditorMode, setEditorMode] = useState<boolean>(false);
  
  return (
    <EditorBackgroundContext.Provider value={{ isEditorMode, setEditorMode }}>
      {children}
    </EditorBackgroundContext.Provider>
  );
};

export const useEditorBackground = (): EditorBackgroundContextType => {
  return useContext(EditorBackgroundContext);
};