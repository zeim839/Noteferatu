// A Markdown Abstract Syntax Tree node.
export type Node =
  | { type: "root" } & BasicContainerBlock
  | { type: "blockquote" } & BasicContainerBlock
  | { type: "footnoteDefinition" } & FootnoteDefinition
  | { type: "list" } & List
  | { type: "break" } & Positional
  | { type: "inlineCode" } & BasicLeafBlock
  | { type: "inlineMath" } & BasicLeafBlock
  | { type: "delete" } & BasicContainerBlock
  | { type: "emphasis" } & BasicContainerBlock
  | { type: "footnoteReference" }  & FootnoteReference
  | { type: "html" } & BasicLeafBlock
  | { type: "image" } & Image
  | { type: "imageReference" } & ImageReference
  | { type: "link" } & Link
  | { type: "linkReference" } & LinkReference
  | { type: "strong" } & BasicContainerBlock
  | { type: "text" } & BasicLeafBlock
  | { type: "code" } & Code
  | { type: "math" } & Math
  | { type: "heading" } & Heading
  | { type: "table" } & Table
  | { type: "thematicBreak" } & Positional
  | { type: "tableRow" } & BasicContainerBlock
  | { type: "tableCell" } & BasicContainerBlock
  | { type: "listItem" } & ListItem
  | { type: "definition" } & Definition
  | { type: "paragraph" } & BasicContainerBlock


// Container blocks may contain other blocks.
export type BasicContainerBlock = {

  // Content model.
  children: Array<Node>

  // Positional Info.
  position?: Position
}

// Leaf blocks may only contain inline string data.
export type BasicLeafBlock = {

  // Content model.
  value: string

  // Positional info.
  position?: Position
}

// A Node body that optionally contains positional information.
export type Positional = {
  position?: Position
}

// A Github-Flavored markdown footnote definition.
//
// Markdown Example:
//
// > | [^a]: b
//   ^^^^^^^
export type FootnoteDefinition = {

  // Content model.
  children: Array<Node>

  // Positional info.
  position?: Position

  // Value that can match another node.
  //
  // identifier is a source value: character escapes and character
  // references are not parsed. Its value must be normalized.
  identifier: string

  // label is a string value: it works just like title on a link or a
  // lang on code: character escapes and character references are parsed.
  label?: string
}

// List root definition. See also: `ListItem`.
//
// Markdown Example:
//
// > | * a
//     ^^^
export type List = {

  // Content model.
  children: Array<Node>

  // Positional info.
  position?: Position

  // Ordered (true) or unordered (false).
  ordered: boolean

  // Starting number of the list. None when unordered.
  start?: number

  // One or more of its children are separated with a blank line from its
  // siblings (when true), or not (when false).
  spread: boolean
}

// GFM Footnote reference.
//
// Markdown Example:
//
// > | [^a]
//     ^^^^
export type FootnoteReference = {

  // Positional info.
  position?: Position

  // Value that can match another node.
  //
  // identifier is a source value: character escapes and character
  // references are not parsed. Its value must be normalized.
  identifier: string

  // label is a string value: it works just like title on a link or a
  // lang on code: character escapes and character references are parsed.
  label?: string
}

// Image node.
//
// Markdown Example:
//
// > | ![a](b)
//     ^^^^^^^
export type Image = {

  // Positional info.
  position?: Position

  // Equivalent content for environments that cannot represent the
  // node as intended.
  alt: string

  // URL to the referenced resource.
  url: string

  // Advisory info for the resource, such as something that would be
  // appropriate for a tooltip.
  title?: string
}

// Reference to an image.
//
// Markdown example:
//
// > | ![a]
//     ^^^^
export type ImageReference = {

  // Positional info.
  position?: Position

  // Equivalent content for environments that cannot represent the
  // node as intended.
  alt: string

  // Explicitness of a reference.
  referenceKind: ReferenceKind

  // Value that can match another node.
  //
  // identifier is a source value: character escapes and character
  // references are not parsed. Its value must be normalized.
  identifier: string

  // label is a string value: it works just like title on a link or a
  // lang on code: character escapes and character references are parsed.
  label?: string
}

// Explicitness of a reference.
export type ReferenceKind = 'shortcut' | 'collapsed' | 'full'

// Hyperlink.
//
// Markdown example:
//
// > | [a](b)
//     ^^^^^^
export type Link = {

  // Content model.
  children: Array<Node>

  // Positional info.
  position?: Position

  // URL to the referenced resource.
  url: string

  // Advisory info for the resource, such as something that would be
  // appropriate for a tooltip.
  title?: string
}

// Link Reference.
//
// Markdown Example:
//
// > | [a]
//     ^^^
export type LinkReference = {

  // Content model.
  children: Array<Node>

  // Positional info.
  position?: Position

  // Explicitness of a reference.
  referenceKind: ReferenceKind

  // Value that can match another node.
  //
  // identifier is a source value: character escapes and character
  // references are not parsed. Its value must be normalized.
  identifier: string

  // label is a string value: it works just like title on a link or a
  // lang on code: character escapes and character references are parsed.
  label?: string
}

// Code block inner text.
export type Code = {

  // Content model.
  value: string

  // Positional info.
  position?: Position

  // The language of computer code being marked up.
  lang?: string

  // Custom info relating to the node.
  meta?: string
}

// Math block inner text.
export type Math = {

  // Content model.
  value: string

  // Positional info.
  position?: Position

  // Custom info relating to the node.
  meta?: string
}

// Heading.
export type Heading = {

  // Content Model.
  children: Array<Node>

  // Positional info.
  position?: Position

  // Rank (between 1 and 6, inclusive).
  depth: number
}

// GFM table.
export type Table = {

  // Content Model.
  children: Array<Node>

  // Positional Info.
  position?: Position

  // Represents how cells in columns are aligned.
  align: Array<AlignKind>
}

// Alignment of the phrasing content.
//
// Used to align the contents of table cells within a table.
export type AlignKind = 'left' | 'right' | 'center' | 'none'

// List item contents.
export type ListItem = {

  // Content model.
  children: Array<Node>

  // Positional Info.
  position?: Position

  // The item contains two or more children separated by a blank line
  // (when true), or not (when false).
  spread: boolean

  // GFM: whether the item is done (when true), not done (when false),
  // or indeterminate or not applicable (None).
  checked?: boolean
}

// Footnote/hyperlink definition.
//
// Markdown Example:
//
// > | [a]: b
//     ^^^^^^
export type Definition = {

  // Positional Info.
  position?: Position

  // URL to the reference resource.
  url: string

  // Advisory info for the resource, such as something that would be
  // appropriate for a tooltip.
  title?: string

  // Value that can match another node.
  //
  // identifier is a source value: character escapes and character
  // references are not parsed. Its value must be normalized.
  identifier: string

  // label is a string value: it works just like title on a link or a
  // lang on code: character escapes and character references are parsed.
  label?: string
}

// Defines a position range relative to two points in a document.
export type Position = {
  start: Point
  end: Point
}

// Specifies a line/column position & offset relative to the document.
export type Point = {
  line: number
  column: number
  offset: number
}
