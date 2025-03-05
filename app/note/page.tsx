"use client";

import React, { useEffect, useRef, useState } from "react";
import { basicSetup } from "codemirror";
import { EditorView, keymap, Decoration, DecorationSet, ViewPlugin, ViewUpdate, highlightActiveLine } from "@codemirror/view";
import { EditorState, RangeSetBuilder } from "@codemirror/state";
import { defaultKeymap, indentWithTab } from "@codemirror/commands";
import { markdown } from "@codemirror/lang-markdown";
import { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { useLayout } from "@/components/layout";


const headerTheme = EditorView.theme({
  ".cm-hidden-hash": {
    fontSize: "0",
    color: "transparent",
    width: "0",
    padding: "0",
    margin: "0",
  },
  ".cm-styled-header": {
    fontWeight: "bold",
    color: "#2a9d8f",
  },
  ".cm-styled-header.level-1": { fontSize: "150%", margin: "20px 0" },
  ".cm-styled-header.level-2": { fontSize: "140%", margin: "15px 0" },
  ".cm-styled-header.level-3": { fontSize: "130%", margin: "10px 0" },
  ".cm-styled-header.level-4": { fontSize: "120%", margin: "5px 0" },
  ".cm-styled-header.level-5": { fontSize: "110%", margin: "5px 0" },
  ".cm-styled-header.level-6": { fontSize: "100%", margin: "5px 0" },
});

const headerDecorations = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = this.createDecorations(view);
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.selectionSet || update.viewportChanged) {
        this.decorations = this.createDecorations(update.view);
      }
    }

    createDecorations(view: EditorView) {
      const builder = new RangeSetBuilder<Decoration>();

      for (const { from, to } of view.visibleRanges) {
        let pos = from;
        while (pos < to) {
          const line = view.state.doc.lineAt(pos);
          const lineRange = { from: line.from, to: line.to };

          const isSelectedOrActive = view.state.selection.ranges.some(range => {
            const selectionOverlaps = range.from <= lineRange.to && range.to >= lineRange.from;
            const cursorOnLine = range.from === range.to &&
                                range.from >= lineRange.from &&
                                range.from <= lineRange.to;
            return selectionOverlaps || cursorOnLine;
          });

          if (!isSelectedOrActive) {
            const match = line.text.match(/^(#{1,6})(\s)(.*)/);
            if (match) {
              const hashLevel = match[1].length;
              const hashStart = line.from + match.index!;
              const spaceEnd = hashStart + match[1].length + 1;

              builder.add(
                hashStart,
                spaceEnd,
                Decoration.mark({ class: "cm-hidden-hash" })
              );

              builder.add(
                spaceEnd,
                line.to,
                Decoration.mark({ class: `cm-styled-header level-${hashLevel}` })
              );
            }
          }
          pos = line.to + 1;
        }
      }

      return builder.finish();
    }
  },
  {
    decorations: (v) => v.decorations,
  }
);

const markdownHighlightStyle = HighlightStyle.define([
  { tag: tags.emphasis, fontStyle: "italic", color: "#e76f51" },
  { tag: tags.strong, fontWeight: "bold", color: "#e76f51" },
  { tag: tags.link, textDecoration: "underline", color: "#264653" },
  { tag: tags.quote, fontStyle: "italic", borderLeft: "3px solid #a8dadc", paddingLeft: "4px", color: "#555" },
  { tag: tags.monospace, backgroundColor: "#f4f4f4", fontFamily: "monospace", padding: "0 2px", borderRadius: "3px" },
  { tag: tags.deleted, textDecoration: "line-through", color: "#6c757d" },
  { tag: tags.list, color: "#457b9d" },
  { tag: tags.punctuation, color: "#999" },
]);

export default function CodeEditor() {
  const editorRef = useRef(null);
  const [text, setText] = useState('');
  const { setBackButton } = useLayout()
  console.log(text)

  const onUpdate = EditorView.updateListener.of((v) => {
    setText(v.state.doc.toString());
  });

  useEffect(() => {

    // Show the back button in the navigation layout.
    setBackButton(true)

    if (!editorRef.current) return;

    const state = EditorState.create({
      doc: 'Hello World\n\n',
      extensions: [
        basicSetup,
        keymap.of([...defaultKeymap, indentWithTab]),
        markdown(),
        onUpdate,
        syntaxHighlighting(markdownHighlightStyle),
        EditorView.lineWrapping,
        headerTheme,
        headerDecorations,
        highlightActiveLine(),
      ],
    });

    const view = new EditorView({
      state,
      parent: editorRef.current,
    });

    return () => view.destroy();
  }, []);

  return <div ref={editorRef} style={{ border: "1px solid #ddd", padding: "10px"}} />;
}
