"use client";

import React, { useEffect, useRef, useState } from "react";
import { basicSetup } from "codemirror";
import { EditorView, keymap, Decoration, DecorationSet, ViewPlugin, ViewUpdate, highlightActiveLine } from "@codemirror/view";
import { EditorState, Line, RangeSetBuilder } from "@codemirror/state";
import { defaultKeymap, indentWithTab } from "@codemirror/commands";
import { markdown } from "@codemirror/lang-markdown";
import { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
import { tags } from "@lezer/highlight";


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
  
  ".cm-hidden-bold": {
    fontSize: "0",
    color: "transparent",
    width: "0",
    padding: "0",
    margin: "0",
  },
  ".cm-styled-bold": {
    fontWeight: "bold",
  },
});

type DecorationContext = {
  view: EditorView;
  line: Line;
  lineRange: { from: number; to: number };
  selection: { start: number; end: number } | null;
  cursorOnLine: boolean;
  cursorPositionInLine: number | null;
  builder: RangeSetBuilder<Decoration>;
}

const createDecorationContext = (view: EditorView, line: Line, builder: RangeSetBuilder<Decoration>): DecorationContext => {
  const lineRange = { from: line.from, to: line.to };
  const mainSelection = view.state.selection.main;
  var selection: { start: number; end: number; } | null = null;
  if (mainSelection.from <= lineRange.to && mainSelection.to >= lineRange.from) {
    const start = Math.max(lineRange.from, Math.min(mainSelection.from, mainSelection.to)) - line.from;
    const end = Math.min(lineRange.to, Math.max(mainSelection.from, mainSelection.to)) - line.from;

    selection = { start, end };
  }
  const cursorOnLine = view.state.selection.ranges.some(
    range => {
      return range.from >= lineRange.from && 
             range.from <= lineRange.to
    }
  );
  const cursorPositionInLine = cursorOnLine 
    ? view.state.selection.main.head - line.from 
    : null;

  return {
    view,
    line,
    lineRange,
    selection,
    cursorOnLine,
    cursorPositionInLine,
    builder
  };
};

const decorateHeaders = (context: DecorationContext) => {
  const { line, selection, cursorOnLine, builder } = context;
  const headerMatch = line.text.match(/^(#{1,6})(\s)(.*)/);

  if (headerMatch) {
    const hashLevel = headerMatch[1].length;
    const hashStart = line.from + headerMatch.index!;
    const spaceEnd = hashStart + headerMatch[1].length + 1;
    if (!(selection || cursorOnLine)) {
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
      else {
        builder.add(
          hashStart,
          line.to,
          Decoration.mark({ class: `cm-styled-header level-${hashLevel}` })
        );
      }
    }
}

const decorateBold = (context: DecorationContext) => {
  const { line, cursorPositionInLine, builder } = context;
  const boldMatches = line.text.matchAll(/\*\*(.*?)\*\*/g);
  for (const boldMatch of boldMatches) {
    const start = line.from + boldMatch.index!;
    const end = start + boldMatch[0].length;
    const intersectingSelection = context.selection && context.selection.start < end - line.from && context.selection.end > start - line.from;
    builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-styled-bold" })
      );
    if (!intersectingSelection && !(cursorPositionInLine && cursorPositionInLine >= boldMatch.index! && cursorPositionInLine <= boldMatch.index! + boldMatch[0].length)) {
      // hide the asterisks
      const firstAsteriskSet = boldMatch.index! + line.from;
      const secondAsteriskSet = line.from + boldMatch.index! + boldMatch[0].length - 2;
      builder.add(
        firstAsteriskSet,
        firstAsteriskSet + 2,
        Decoration.mark({ class: "cm-hidden-bold" })
      );
      builder.add(
        secondAsteriskSet,
        secondAsteriskSet + 2,
        Decoration.mark({ class: "cm-hidden-bold" })
      );
    }
  }
}

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
          const context = createDecorationContext(view, line, builder);
          decorateHeaders(context);
          decorateBold(context);

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
  console.log(text)

  const onUpdate = EditorView.updateListener.of((v) => {
    setText(v.state.doc.toString());
  });

  useEffect(() => {
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
