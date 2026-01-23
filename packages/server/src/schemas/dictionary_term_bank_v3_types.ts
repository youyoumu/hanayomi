/**
 * Dictionary term bank V3 types
 * Represents structured dictionary data with complex nested definitions
 */

export type DictionaryTermBankV3 = Array<DictionaryTermBankV3Row>;

/**
 * Main dictionary term type - represents a tuple structure
 */
export type DictionaryTermBankV3Row = [
  string, // The text for the term
  string, // Reading of the term, or empty string if same as term
  string | null, // Space-separated tags for definition, null means no tags
  string, // Space-separated rule identifiers for definition validation
  number, // Score for popularity/ranking (negative = rare, positive = frequent)
  Definition[], // Array of definitions for the term
  number, // Sequence number for terms with same sequence
  string, // Space-separated tags for the term
];

/**
 * Definition types - can be text, detailed, or deinflection
 */
export type Definition =
  | string // Single text definition
  | DetailedDefinition // Detailed definition with structure
  | Deinflection; // Deinflection information to uninflected term

/**
 * Detailed definition variants, tagged by type
 */
export type DetailedDefinition =
  | { type: "text"; text: TextDefinition }
  | { type: "image"; image: ImageDefinition }
  | { type: "structured-content"; structuredContent: StructuredContentDefinition };

/**
 * Text definition - simple string content
 */
export type TextDefinition = {
  text: string;
};

/**
 * Image definition with display and styling options
 */
export type ImageDefinition = {
  path: string;
  width?: number | null;
  height?: number | null;
  title?: string | null;
  alt?: string | null;
  description?: string | null;
  pixelated?: boolean;
  imageRendering?: "auto" | "pixelated" | "crisp-edges";
  appearance?: "auto" | "monochrome";
  background?: boolean;
  collapsed?: boolean;
  collapsible?: boolean;
};

/**
 * Structured content definition
 */
export type StructuredContentDefinition = {
  content: StructuredContent;
};

/**
 * Deinflection information for terms
 */
export type Deinflection = {
  uninflectedTerm: string;
  inflectionRules: string[];
};

// Note: inflectedTerm is represented as string[] in TypeScript (type alias in Rust)

/**
 * Structured content - recursive content structure
 */
export type StructuredContent =
  | string // Text node
  | StructuredContent[] // Array node
  | StructuredContentObject; // Object node

/**
 * Structured content object - represents HTML elements with tags
 */
export type StructuredContentObject =
  | ({ tag: "br" } & BreakFields)
  | ({ tag: "ruby" } & RubyFields)
  | ({ tag: "rt" } & RtFields)
  | ({ tag: "rp" } & RpFields)
  | ({ tag: "table" } & TableFields)
  | ({ tag: "thead" } & TheadFields)
  | ({ tag: "tbody" } & TbodyFields)
  | ({ tag: "tfoot" } & TfootFields)
  | ({ tag: "tr" } & TrFields)
  | ({ tag: "td" } & TdFields)
  | ({ tag: "th" } & ThFields)
  | ({ tag: "span" } & SpanFields)
  | ({ tag: "div" } & DivFields)
  | ({ tag: "ol" } & OlFields)
  | ({ tag: "ul" } & UlFields)
  | ({ tag: "li" } & LiFields)
  | ({ tag: "details" } & DetailsFields)
  | ({ tag: "summary" } & SummaryFields)
  | ({ tag: "img" } & ImgFields)
  | ({ tag: "a" } & LinkFields);

/**
 * Ruby annotation fields
 */
export type RubyFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * RT (ruby text) fields
 */
export type RtFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * RP (ruby parenthesis) fields
 */
export type RpFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table fields
 */
export type TableFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table head fields
 */
export type TheadFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table body fields
 */
export type TbodyFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table foot fields
 */
export type TfootFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table row fields
 */
export type TrFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
};

/**
 * Table cell fields
 */
export type TdFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  lang?: string | null;
};

/**
 * Table header cell fields
 */
export type ThFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  lang?: string | null;
};

/**
 * Span fields
 */
export type SpanFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
};

/**
 * Div fields
 */
export type DivFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Ordered list fields
 */
export type OlFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Unordered list fields
 */
export type UlFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * List item fields
 */
export type LiFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Details fields
 */
export type DetailsFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Summary fields
 */
export type SummaryFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Image fields (duplicate for img tag)
 */
export type ImgFields = {
  path: string;
  data?: Record<string, string> | null;
  width?: number | null;
  height?: number | null;
  title?: string | null;
  alt?: string | null;
  description?: string | null;
  pixelated?: boolean;
  imageRendering?: "auto" | "pixelated" | "crisp-edges";
  appearance?: "auto" | "monochrome";
  background?: boolean;
  collapsed?: boolean;
  collapsible?: boolean;
  verticalAlign?:
    | "baseline"
    | "sub"
    | "super"
    | "text-top"
    | "text-bottom"
    | "middle"
    | "top"
    | "bottom";
  border?: string | null;
  borderRadius?: string | null;
  sizeUnits?: "px" | "em";
};

/**
 * Link element fields
 */
export type LinkFields = {
  href: string;
  content?: StructuredContent | null;
  lang?: string | null;
};

/**
 * Base fields for empty tags like <br>
 */
export type BreakFields = {
  data?: Record<string, string> | null;
};

/**
 * Generic container fields for HTML elements
 */
export type ContainerFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  lang?: string | null;
};

/**
 * Table-specific fields with colspan/rowspan support
 */
export type TableElementFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  colSpan?: number | null;
  rowSpan?: number | null;
  style?: StructuredContentStyle | null;
  lang?: string | null;
};

/**
 * Styled container fields with additional styling options
 */
export type StyledContainerFields = {
  content?: StructuredContent | null;
  data?: Record<string, string> | null;
  style?: StructuredContentStyle | null;
  title?: string | null;
  open?: boolean | null;
  lang?: string | null;
};

/**
 * Image element fields with full styling support
 */
export type ImageFields = {
  path: string;
  data?: Record<string, string> | null;
  width?: number | null;
  height?: number | null;
  title?: string | null;
  alt?: string | null;
  description?: string | null;
  pixelated?: boolean;
  imageRendering?: "auto" | "pixelated" | "crisp-edges";
  appearance?: "auto" | "monochrome";
  background?: boolean;
  collapsed?: boolean;
  collapsible?: boolean;
  verticalAlign?:
    | "baseline"
    | "sub"
    | "super"
    | "text-top"
    | "text-bottom"
    | "middle"
    | "top"
    | "bottom";
  border?: string | null;
  borderRadius?: string | null;
  sizeUnits?: "px" | "em";
};

/**
 * Generic data type for element attributes
 */
export type StructuredContentData = Record<string, string>;

/**
 * Text decoration line type
 */
export type TextDecorationLine = string | string[];

/**
 * Structured content style properties
 */
export type StructuredContentStyle = {
  fontStyle?: "normal" | "italic" | null;
  fontWeight?: "normal" | "bold" | null;
  fontSize?: string | null;
  color?: string | null;
  background?: string | null;
  backgroundColor?: string | null;
  textDecorationLine?: TextDecorationLine | null;
  textDecorationStyle?: "solid" | "double" | "dotted" | "dashed" | "wavy" | null;
  textDecorationColor?: string | null;
  borderColor?: string | null;
  borderStyle?: string | null;
  borderRadius?: string | null;
  borderWidth?: string | null;
  clipPath?: string | null;
  verticalAlign?:
    | "baseline"
    | "sub"
    | "super"
    | "text-top"
    | "text-bottom"
    | "middle"
    | "top"
    | "bottom"
    | null;
  textAlign?:
    | "start"
    | "end"
    | "left"
    | "right"
    | "center"
    | "justify"
    | "justify-all"
    | "match-parent"
    | null;
  textEmphasis?: string | null;
  textShadow?: string | null;
  margin?: string | null;
  marginTop?: NumberOrString | null;
  marginLeft?: NumberOrString | null;
  marginRight?: NumberOrString | null;
  marginBottom?: NumberOrString | null;
  padding?: string | null;
  paddingTop?: string | null;
  paddingLeft?: string | null;
  paddingRight?: string | null;
  paddingBottom?: string | null;
  wordBreak?: "normal" | "break-all" | "keep-all" | null;
  whiteSpace?: string | null;
  cursor?: string | null;
  listStyleType?: string | null;
};

/**
 * Number or string union for flexible numeric/string values
 */
export type NumberOrString = { number: number } | { string: string };
