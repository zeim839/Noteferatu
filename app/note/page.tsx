"use client";

import React, { useEffect, useRef, useState, useMemo } from "react";
import { basicSetup } from "codemirror";
import { EditorView, keymap, Decoration, DecorationSet, ViewPlugin, ViewUpdate, WidgetType } from "@codemirror/view";
import { EditorState, Line, RangeSetBuilder } from "@codemirror/state";
import { defaultKeymap, indentWithTab } from "@codemirror/commands";
import { markdown } from "@codemirror/lang-markdown";
import { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
import { tags } from "@lezer/highlight";


const codeMirrorTheme = EditorView.theme({
  ".cm-hidden-characters": {
    fontSize: "0",
    color: "transparent",
    width: "0",
    padding: "0",
    margin: "0",
  },
  ".cm-styled-header": {
    fontWeight: "500",
  },
  ".cm-styled-header.level-1": { fontSize: "175%" },
  ".cm-styled-header.level-2": { fontSize: "150%" },
  ".cm-styled-header.level-3": { fontSize: "135%" },
  ".cm-styled-header.level-4": { fontSize: "120%" },
  ".cm-styled-header.level-5": { fontSize: "110%" },
  ".cm-styled-header.level-6": { fontSize: "100%" },

  ".cm-styled-bold": {
    fontWeight: "bold",
  },

  ".cm-styled-link": {
    color: "#3477eb",
  },
  ".cm-styled-link *": {
    color: "inherit"
  },

  ".cm-styled-quote": {
    borderLeft: "3px solid #a8dadc"
  },

  ".cm-lineNumbers": {
    width: "0"
  },
  ".cm-scroller": {
    fontFamily: "'Iosevka Comfy', monospace"
  },
  "&.cm-focused": {
      outline: "none",
  },
  ".cm-activeLineGutter, .cm-gutters": {
    backgroundColor: "transparent"
  },
  ".cm-gutters": {
    borderRight: "none"
  },
  ".cm-line-h1": {
    borderBottom: "1px solid grey",
    paddingBottom: "10px",
    marginBottom: "10px"
  },
  ".cm-line-higher-headers": {
    paddingTop: "5px",
    paddingBottom: "5px",
    marginBottom: "5px",
    display: "flex",
    justifyContent: "start"
  },
  
});

type DecorationContext = {
  view: EditorView;
  line: Line;
  lineRange: { from: number; to: number };
  selection: { start: number; end: number } | null;
  cursorOnLine: boolean;
  builder: RangeSetBuilder<Decoration>;
}

const createDecorationContext = (view: EditorView, line: Line, builder: RangeSetBuilder<Decoration>): DecorationContext => {
  const lineRange = { from: line.from, to: line.to };
  const mainSelection = view.state.selection.main;
  let selection: { start: number; end: number; } | null = null;
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

  return {
    view,
    line,
    lineRange,
    selection,
    cursorOnLine,
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
      if (hashLevel == 1) {
        builder.add(
          line.from,
          line.from,
          Decoration.line({ class: "cm-line-h1" })
        );
      } else {
        builder.add(
          line.from,
          line.from,
          Decoration.line({ class: "cm-line-higher-headers" })
        );
      }
    
      builder.add(
        hashStart,
        spaceEnd,
        Decoration.mark({ class: "cm-hidden-characters" })
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
  const { line, selection, builder } = context;
  const boldMatches = line.text.matchAll(/\*\*(.*?)\*\*/g);
  for (const boldMatch of boldMatches) {
    const start = line.from + boldMatch.index!;
    const end = start + boldMatch[0].length;
    const intersectingSelection = selection && selection.start < end - line.from && selection.end > start - line.from;
    builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-styled-bold" })
      );
    if (!intersectingSelection) {
      // hide the asterisks
      const firstAsteriskSet = boldMatch.index! + line.from;
      const secondAsteriskSet = line.from + boldMatch.index! + boldMatch[0].length - 2;
      builder.add(
        firstAsteriskSet,
        firstAsteriskSet + 2,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
      builder.add(
        secondAsteriskSet,
        secondAsteriskSet + 2,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
    }
  }
}

class LinkWidget extends WidgetType {
  constructor(readonly text: string, readonly url: string) {
    super();
  }

  toDOM() {
    const link = document.createElement("a");
    link.textContent = this.text;
    link.href = this.url;
    link.target = "_blank";
    link.rel = "noopener noreferrer";
    link.classList.add("cm-styled-link");
    return link;
  }
}

const decorateLink = (context: DecorationContext) => {
  const { line, selection, builder } = context;
  const linkMatches = line.text.matchAll(/(?<!\!)\[([^\]]+)\]\(([^)]+)\)/g);

  for (const linkMatch of linkMatches) {
    const start = line.from + linkMatch.index!;
    const textStart = start + 1;
    const textEnd = start + linkMatch[1].length + 1;
    const end = start + linkMatch[0].length;
    const intersectingSelection = selection && selection.start < end - line.from && selection.end > start - line.from;
    
    builder.add(
      start,
      end,
      Decoration.mark({ class: "cm-styled-link" })
    );

    if (!intersectingSelection) {
      builder.add(
        start,
        start + 1,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
      builder.add(
        textStart,
        textEnd,
        Decoration.widget({ widget: new LinkWidget(linkMatch[1], linkMatch[2]) })
      );
      builder.add(
        textEnd,
        end,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
    }
  }
}

const decorateQuote = (context: DecorationContext) => {
  const { line, selection, cursorOnLine, builder } = context;
  const quoteMatch = line.text.match(/>\s.+/);

  if (quoteMatch) {
    const start = line.from + quoteMatch.index!;
    const end = start + quoteMatch[0].length;
    if (!(selection || cursorOnLine)) {
      builder.add(
        start,
        start + 1,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
      builder.add(
        start + 1,
        end,
        Decoration.mark({ class: "cm-styled-quote" })
      );
    }
  }
}

class ImageWidget extends WidgetType {
  constructor(readonly src: string, readonly altText: string) {
    super();
  }

  toDOM() {
    const img = document.createElement("img");
    img.src = this.src;
    img.alt = this.altText;

    return img;
  }
}

const decorateImage = (context: DecorationContext) => {
  const { line, selection, builder } = context;
  const imageMatch = line.text.match(/!\[(.+?)\]\((.+?)(?:\s"(.+?)")?\)(?=\s|$)/);

  if (imageMatch) {
    const start = line.from + imageMatch.index!;
    const end = start + imageMatch[0].length;

    const intersectingSelection = selection && selection.start < end - line.from && selection.end > start - line.from;
    if(!intersectingSelection) {
      builder.add(
        start,
        end,
        Decoration.mark({ class: "cm-hidden-characters" })
      );
      builder.add(
        end,
        end,
        Decoration.widget({ widget: new ImageWidget(imageMatch[2], imageMatch[1]) })
      );
    }
  }
}

const Decorations = ViewPlugin.fromClass(
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
          decorateLink(context);
          decorateQuote(context);
          decorateImage(context);

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
  { tag: tags.quote, fontStyle: "italic", color: "#555" },
  { tag: tags.monospace, backgroundColor: "#f4f4f4", fontFamily: "monospace", padding: "0 2px", borderRadius: "3px" },
  { tag: tags.deleted, textDecoration: "line-through", color: "#6c757d" },
  { tag: tags.list, color: "#457b9d" },
  { tag: tags.punctuation, color: "#999" },
]);

// TODO: make it so there is always a line underneath the cursor 
export default function CodeEditor() {
  const editorRef = useRef(null);
  const [text, setText] = useState('');
  console.log(text)

  const onUpdate = useMemo(
    () =>
      EditorView.updateListener.of((v) => {
        setText(v.state.doc.toString());
      }),
    []
  );
  

  useEffect(() => {
    if (!editorRef.current) return;

    document.body.style.backgroundColor = '#FBF9F3';

    const state = EditorState.create({
      doc: 'Hello World\n\n',
      extensions: [
        basicSetup,
        keymap.of([...defaultKeymap, indentWithTab]),
        markdown(),
        onUpdate,
        syntaxHighlighting(markdownHighlightStyle),
        EditorView.lineWrapping,
        codeMirrorTheme,
        Decorations
      ],
    });

    const view = new EditorView({
      state,
      parent: editorRef.current,
    });

    return () => view.destroy();
  }, [onUpdate]);

  return <div ref={editorRef} style={{ padding: "10px", maxWidth: "800px", width: "100%", margin: "0 auto"}} />;
}
