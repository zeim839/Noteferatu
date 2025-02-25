"use client";

import React, { useEffect, useRef, useState } from "react";
import {basicSetup} from "codemirror"
import { EditorView, keymap, MatchDecorator, Decoration, WidgetType, DecorationSet, ViewPlugin, ViewUpdate } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap, indentWithTab } from "@codemirror/commands";
import {markdown} from "@codemirror/lang-markdown"
import {syntaxHighlighting, HighlightStyle} from "@codemirror/language"
import {tags} from "@lezer/highlight"  


class RenderedMarkdownWidget extends WidgetType {
  text: string;
  constructor(text: string) {
      super();
      this.text = text;
  }
  toDOM(view: EditorView): HTMLElement {
    let element = document.createElement('span');
    element.className = 'rendered-markdown-widget';
    element.textContent = this.text;

    return element;
  }
}

export default function CodeEditor() {
  const editorRef = useRef(null);
  const [text, setText] = useState('')

  const onUpdate = EditorView.updateListener.of((v) => {
    setText(v.state.doc.toString())
  })

  const placeholderMatcher = new MatchDecorator({
    regexp: /^# (.*)/g,
    decoration: match => Decoration.replace({
      widget: new RenderedMarkdownWidget(match[1]),
    })
  });

  useEffect(() => {
    if (!editorRef.current) return;

    const markdownHighlightStyle = HighlightStyle.define([
      // Headings
      { tag: tags.heading1, fontSize: "150%", fontWeight: "bold", color: "#2a9d8f", textDecoration: 'none' },
      { tag: tags.heading2, fontSize: "140%", fontWeight: "bold", color: "#2a9d8f" },
      { tag: tags.heading3, fontSize: "130%", fontWeight: "bold", color: "#2a9d8f" },
      { tag: tags.heading4, fontSize: "120%", fontWeight: "bold", color: "#2a9d8f" },
      { tag: tags.heading5, fontSize: "110%", fontWeight: "bold", color: "#2a9d8f" },
      { tag: tags.heading6, fontSize: "100%", fontWeight: "bold", color: "#2a9d8f" },
      
      // Emphasis and strong (italic and bold)
      { tag: tags.emphasis, fontStyle: "italic", color: "#e76f51" },
      { tag: tags.strong, fontWeight: "bold", color: "#e76f51" },
      
      // Links
      { tag: tags.link, textDecoration: "underline", color: "#264653" },
      
      // Blockquotes
      { tag: tags.quote, fontStyle: "italic", borderLeft: "3px solid #a8dadc", paddingLeft: "4px", color: "#555" },
      
      // Inline code and code blocks
      { tag: tags.monospace, backgroundColor: "#f4f4f4", fontFamily: "monospace", padding: "0 2px", borderRadius: "3px" },
      
      // Strikethrough (if your Markdown supports it)
      { tag: tags.deleted, textDecoration: "line-through", color: "#6c757d" },
      
      { tag: tags.list, color: "#457b9d" },
      { tag: tags.punctuation, color: "#999" },
    ]);

    const placeholders = ViewPlugin.fromClass(class {
      placeholders: DecorationSet
      constructor(view: EditorView) {
        this.placeholders = placeholderMatcher.createDeco(view)
      }
      update(update: ViewUpdate) {
        this.placeholders = placeholderMatcher.updateDeco(update, this.placeholders)
      }
    }, {
      decorations: instance => instance.placeholders,
      provide: plugin => EditorView.atomicRanges.of(view => {
        return view.plugin(plugin)?.placeholders || Decoration.none
      })
    })

    // Create a new CodeMirror 6 EditorState
    const state = EditorState.create({
      doc: 'Hello [[pattern1]]',
      extensions: [basicSetup, keymap.of([...defaultKeymap, indentWithTab]), markdown(), onUpdate,
      syntaxHighlighting(markdownHighlightStyle), EditorView.lineWrapping, placeholders],
    });

    // Attach the editor to the div
    const view = new EditorView({
      state,
      parent: editorRef.current,
    });

    return () => view.destroy(); // Cleanup on unmount
  }, []);

  return <div ref={editorRef} style={{ border: "1px solid #ddd", padding: "10px" }} />;
}
