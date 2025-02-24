"use client";

import React, { useEffect, useRef } from "react";
import {basicSetup} from "codemirror"
import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap } from "@codemirror/commands";

export default function CodeEditor() {
  const editorRef = useRef(null);

  useEffect(() => {
    if (!editorRef.current) return;

    // Create a new CodeMirror 6 EditorState
    const startState = EditorState.create({
      doc: 'Hello World',
      extensions: [basicSetup, keymap.of(defaultKeymap)],
    });

    // Attach the editor to the div
    const view = new EditorView({
      state: startState,
      parent: editorRef.current,
    });

    return () => view.destroy(); // Cleanup on unmount
  }, []);

  return <div ref={editorRef} style={{ border: "1px solid #ddd", padding: "10px" }} />;
}
