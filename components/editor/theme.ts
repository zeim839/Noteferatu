import { EditorView } from '@codemirror/view'
import { HighlightStyle } from '@codemirror/language'
import { tags } from '@lezer/highlight'
export const codeMirrorTheme = EditorView.theme({
    '.cm-hidden-characters': {
        fontSize: '0',
        color: 'transparent',
        width: '0',
        padding: '0',
        margin: '0',
    },
    '.cm-styled-header': {
        fontWeight: '500',
    },
    '.cm-styled-header.level-1': {
        fontSize: '175%',
    },
    '.cm-styled-header.level-2': {
        fontSize: '150%',
    },
    '.cm-styled-header.level-3': {
        fontSize: '135%',
    },
    '.cm-styled-header.level-4': {
        fontSize: '120%',
    },
    '.cm-styled-header.level-5': {
        fontSize: '110%',
    },
    '.cm-styled-header.level-6': {
        fontSize: '100%',
    },
    '.cm-styled-bold': {
        fontWeight: 'bold',
    },
    '.cm-styled-italic': {
        fontStyle: 'italic',
    },
    '.cm-styled-bold-italic': {
        fontStyle: 'italic',
        fontWeight: 'bold',
    },
    '.cm-styled-link': {
        color: '#3477eb',
    },
    '.cm-styled-link *': {
        color: 'inherit',
    },
    '.cm-styled-quote': {
        position: 'relative',
        color: 'transparent',
        display: 'inline-block',
    },
    '.cm-styled-quote-text': {
        color: '#555',
        fontStyle: 'italic',
    },
    '.cm-styled-quote::before': {
        content: '""',
        position: 'absolute',
        top: '0',
        bottom: '0',
        left: '-5px',
        width: '3px',
        backgroundColor: '#a8dadc',
    },
    '.cm-styled-quote-focused': {
        position: 'relative',
        color: '#555',
        display: 'inline-block',
    },
    '.cm-styled-quote-focused::before': {
        content: '""',
        position: 'absolute',
        top: '0',
        bottom: '0',
        left: '-5px',
        width: '3px',
        backgroundColor: '#a8dadc',
    },
    '.cm-lineNumbers': {
        width: '0',
    },
    '.cm-scroller': {
        fontFamily: "'Iosevka Comfy', monospace",
    },
    '&.cm-focused': {
        outline: 'none',
    },
    '.cm-activeLineGutter, .cm-gutters': {
        backgroundColor: 'transparent',
    },
    '.cm-gutters': {
        borderRight: 'none',
    },
    '.cm-line-h1': {
        borderBottom: '1px solid grey',
        paddingBottom: '10px',
        marginBottom: '10px',
    },
    '.cm-line-higher-headers': {
        paddingTop: '5px',
        paddingBottom: '5px',
        marginBottom: '5px',
        display: 'flex',
        justifyContent: 'start',
    },
    '&:not(.cm-focused) .cm-activeLine': {
        background: 'transparent',
    },
    '.cm-widgetBuffer': {
        pointerEvents: 'none',
        position: 'relative',
        zIndex: '0',
    },
    '.cm-hide-image-line': {
        color: 'transparent',
        fontSize: '0',
        display: 'block',
        width: '100%',
    },
    '.cm-line': {
        padding: '0',
    },
    '.cm-styled-inline-code': {
        fontFamily: 'monospace',
        backgroundColor: 'rgba(230, 230, 230, 0.8)',
        borderRadius: '4px',
        padding: '1px 4px',
        position: 'relative',
        zIndex: '0',
    },
    '.cm-styled-fenced-code': {
        fontFamily: 'monospace',
        backgroundColor: 'rgba(230, 230, 230, 0.8)',
        textIndent: '7px',
        overflowX: 'auto',
        display: 'block',
        whiteSpace: 'pre',
        userSelect: 'text',
    },
    '.cm-styled-fenced-code-active': {
        fontFamily: 'monospace',
        backgroundColor: 'rgba(220, 220, 220, 0.8)',
        textIndent: '7px',
        overflowX: 'auto',
        display: 'block',
        whiteSpace: 'pre',
        userSelect: 'text',
    },
    '.cm-styled-inline-hr': {
        height: '2px',
        backgroundColor: '#888',
        width: '100%',
        border: 'none',
        marginTop: '-10px',
    },
    '.cm-styled-horizontal-rule': {
        color: '#aaa',
        fontStyle: 'italic',
    },
})

export const markdownHighlightStyle = HighlightStyle.define([
    { tag: tags.deleted, textDecoration: 'line-through', color: '#6c757d' },
    { tag: tags.list, color: '#457b9d' },
    { tag: tags.punctuation, color: '#999' },
])
