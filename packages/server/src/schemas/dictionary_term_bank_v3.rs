use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

pub type DictionaryTermBankV3 = Vec<DictionaryTermBankV3Row>;

#[derive(Deserialize, Serialize, Debug)]
/// Information about a single term.
pub struct DictionaryTermBankV3Row(
    /// The text for the term.
    String,
    /// Reading of the term, or an empty string if the reading is the same as the term.
    String,
    /// String of space-separated tags for the definition. An empty string is treated as no tags.
    Option<String>,
    /// String of space-separated rule identifiers for the definition which is used to validate deinflection. An empty string should be used for words which aren't inflected.
    String,
    /// Score used to determine popularity. Negative values are more rare and positive values are more frequent. This score is also used to sort search results.
    f32,
    /// Array of definitions for the term.
    Vec<Term>,
    /// Sequence number for the term. Terms with the same sequence number can be shown together when the "resultOutputMode" option is set to "merge".
    i32,
    /// String of space-separated tags for the term. An empty string is treated as no tags.
    String,
);

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Term {
    /// Single definition for the term.
    Definition(String),
    /// Single detailed definition for the term.
    DetailedDefinition(Box<DetailedDefinition>),
    /// Deinflection of the term to an uninflected term.
    Deinflection(Deinflection),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum DetailedDefinition {
    Text(TextDefinition),
    Image(ImageDefinition),
    StructuredContent(StructuredContentDefinition),
}

#[derive(Deserialize, Serialize, Debug)]
struct Deinflection(
    /// The uninflected term.
    String,
    /// A chain of inflection rules that produced the inflected term
    Vec<InflectedTerm>,
);

/// A single inflection rule.
type InflectedTerm = String;

#[derive(Deserialize, Serialize, Debug)]
struct TextDefinition {
    /// Single definition for the term.
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
struct ImageDefinition {
    /// Path to the image file in the archive.
    path: String,
    /// Preferred width of the image.
    #[validate(minimum = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i32>,
    /// Preferred height of the image.
    #[validate(minimum = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i32>,
    /// Hover text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// Alt text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<String>,
    /// Description of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Whether or not the image should appear pixelated at sizes larger than the image's native resolution.
    #[serde(default)]
    pixelated: bool,
    /// Controls how the image is rendered. The value of this field supersedes the pixelated field.
    #[validate(enumerate = ["auto", "pixelated", "crisp-edges"])]
    #[serde(default = "default_auto")]
    image_rendering: String,
    /// Controls the appearance of the image. The "monochrome" value will mask the opaque parts of the image using the current text color.
    #[validate(enumerate = ["auto", "monochrome"])]
    #[serde(default = "default_auto")]
    appearance: String,
    /// Whether or not a background color is displayed behind the image.
    #[serde(default = "default_true")]
    background: bool,
    /// Whether or not the image is collapsed by default.
    #[serde(default)]
    collapsed: bool,
    /// Whether or not the image can be collapsed.
    #[serde(default = "default_true")]
    collapsible: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct StructuredContentDefinition {
    /// Single definition for the term using a structured content object.
    content: Box<StructuredContent>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum StructuredContent {
    /// Represents a text node.
    Text(String),
    /// An array of child content.
    Array(Vec<StructuredContent>),
    Object(Box<StructuredContentObject>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "tag", rename_all = "lowercase")]
pub enum StructuredContentObject {
    Br(BreakFields),

    Ruby(ContainerFields),
    Rt(ContainerFields),
    Rp(ContainerFields),
    Table(ContainerFields),
    Thead(ContainerFields),
    Tbody(ContainerFields),
    Tfoot(ContainerFields),
    Tr(ContainerFields),

    Td(TableElementFields),
    Th(TableElementFields),

    Span(StyledContainerFields),
    Div(StyledContainerFields),
    Ol(StyledContainerFields),
    Ul(StyledContainerFields),
    Li(StyledContainerFields),
    Details(StyledContainerFields),
    Summary(StyledContainerFields),

    Img(ImageFields),

    A(LinkFields),
}

impl StructuredContentObject {
    /// Returns the tag name associated with this variant.
    pub fn _tag(&self) -> &'static str {
        match self {
            Self::Br(_) => "br",
            Self::Ruby(_) => "ruby",
            Self::Rt(_) => "rt",
            Self::Rp(_) => "rp",
            Self::Table(_) => "table",
            Self::Thead(_) => "thead",
            Self::Tbody(_) => "tbody",
            Self::Tfoot(_) => "tfoot",
            Self::Tr(_) => "tr",
            Self::Td(_) => "td",
            Self::Th(_) => "th",
            Self::Span(_) => "span",
            Self::Div(_) => "div",
            Self::Ol(_) => "ol",
            Self::Ul(_) => "ul",
            Self::Li(_) => "li",
            Self::Details(_) => "details",
            Self::Summary(_) => "summary",
            Self::Img(_) => "img",
            Self::A(_) => "a",
        }
    }
}

// --- Data Structures for the Variants ---

/// Empty tags.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct BreakFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<StructuredContentData>,
}

/// Generic container tags.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct ContainerFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<StructuredContentData>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}

/// Table tags.
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TableElementFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<StructuredContentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 1)]
    col_span: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 1)]
    row_span: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<StructuredContentStyle>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}

/// Container tags supporting configurable styles.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct StyledContainerFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<StructuredContentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<StructuredContentStyle>,
    /// Hover text for the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// Whether or not the details element is open by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    open: Option<bool>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}

/// Image tag.
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImageFields {
    /// Path to the image file in the archive.
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Box<StructuredContentData>>,
    /// Preferred width of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    width: Option<f32>,
    /// Preferred height of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    height: Option<f32>,
    /// Hover text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// Alt text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<String>,
    /// Description of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Whether or not the image should appear pixelated at sizes larger than the image's native resolution.
    #[serde(default)]
    pixelated: bool,
    /// Controls how the image is rendered. The value of this field supersedes the pixelated field.
    #[serde(default = "default_auto")]
    image_rendering: String,
    /// Controls the appearance of the image. The "monochrome" value will mask the opaque parts of the image using the current text color.
    #[serde(default = "default_auto")]
    appearance: String,
    /// Whether or not a background color is displayed behind the image.
    #[serde(default = "default_true")]
    background: bool,
    /// Whether or not the image is collapsed by default.
    #[serde(default)]
    collapsed: bool,
    /// Whether or not the image can be collapsed.
    #[serde(default)]
    collapsible: bool,
    /// The vertical alignment of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<String>,
    /// Shorthand for border width, style, and color.
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,
    /// Roundness of the corners of the image's outer border edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<String>,
    /// The units for the width and height.
    #[serde(skip_serializing_if = "Option::is_none")]
    size_units: Option<String>,
}

/// Link tag.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct LinkFields {
    #[validate(pattern = r"^(?:https?:|\?)[\w\W]*")]
    href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<StructuredContent>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}

/// Generic data attributes that should be added to the element.
type StructuredContentData = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
struct StructuredContentStyle {
    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "italic"])]
    font_style: String,

    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "bold"])]
    font_weight: String,

    #[serde(default = "default_medium")]
    font_size: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,

    #[serde(default)]
    text_decoration_line: TextDecorationLine,

    #[serde(default = "default_solid")]
    #[validate(enumerate = ["solid", "double", "dotted", "dashed", "wavy"])]
    text_decoration_style: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_decoration_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clip_path: Option<String>,

    #[serde(default = "default_baseline")]
    #[validate(enumerate = ["baseline", "sub", "super", "text-top", "text-bottom", "middle", "top", "bottom"])]
    vertical_align: String,

    #[serde(default = "default_start")]
    #[validate(enumerate = ["start", "end", "left", "right", "center", "justify", "justify-all", "match-parent"])]
    text_align: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_emphasis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<String>,

    #[serde(default)]
    margin_top: NumberOrString,

    #[serde(default)]
    margin_left: NumberOrString,

    #[serde(default)]
    margin_right: NumberOrString,

    #[serde(default)]
    margin_bottom: NumberOrString,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding_top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding_left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding_right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding_bottom: Option<String>,

    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "break-all", "keep-all"])]
    word_break: String,

    #[serde(default = "default_normal")]
    white_space: String,

    #[serde(default = "default_auto")]
    cursor: String,

    #[serde(default = "default_disc")]
    list_style_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum TextDecorationLine {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for TextDecorationLine {
    fn default() -> Self {
        Self::Single("none".to_string())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum NumberOrString {
    Number(f32),
    String(String),
}

impl Default for NumberOrString {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

fn default_normal() -> String {
    "normal".to_string()
}
fn default_medium() -> String {
    "medium".to_string()
}
fn default_solid() -> String {
    "solid".to_string()
}
fn default_baseline() -> String {
    "baseline".to_string()
}
fn default_start() -> String {
    "start".to_string()
}
fn default_auto() -> String {
    "auto".to_string()
}
fn default_disc() -> String {
    "disc".to_string()
}
fn default_true() -> bool {
    true
}

mod r#impl;

#[cfg(test)]
mod test;
