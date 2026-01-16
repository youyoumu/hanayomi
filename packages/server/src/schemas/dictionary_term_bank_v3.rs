use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

pub type DictionaryTermBankV3 = Vec<DictionaryTermBankV3Row>;

#[derive(Deserialize, Serialize, Debug)]
/// Information about a single term.
pub struct DictionaryTermBankV3Row(
    /// The text for the term.
    pub String,
    /// Reading of the term, or an empty string if the reading is the same as the term.
    pub String,
    /// String of space-separated tags for the definition. An empty string is treated as no tags.
    pub Option<String>,
    /// String of space-separated rule identifiers for the definition which is used to validate deinflection. An empty string should be used for words which aren't inflected.
    pub String,
    /// Score used to determine popularity. Negative values are more rare and positive values are more frequent. This score is also used to sort search results.
    pub f32,
    /// Array of definitions for the term.
    pub Vec<Definition>,
    /// Sequence number for the term. Terms with the same sequence number can be shown together when the "resultOutputMode" option is set to "merge".
    pub i32,
    /// String of space-separated tags for the term. An empty string is treated as no tags.
    pub String,
);

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Definition {
    /// Single definition for the term.
    Text(String),
    /// Single detailed definition for the term.
    Detailed(Box<DetailedDefinition>),
    /// Deinflection of the term to an uninflected term.
    Deinflection(Deinflection),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DetailedDefinition {
    Text(TextDefinition),
    Image(ImageDefinition),
    StructuredContent(StructuredContentDefinition),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Deinflection(
    /// The uninflected term.
    pub String,
    /// A chain of inflection rules that produced the inflected term
    pub Vec<InflectedTerm>,
);

/// A single inflection rule.
pub type InflectedTerm = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct TextDefinition {
    /// Single definition for the term.
    text: String,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImageDefinition {
    /// Path to the image file in the archive.
    pub path: String,
    /// Preferred width of the image.
    #[validate(minimum = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Preferred height of the image.
    #[validate(minimum = 1)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// Hover text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Alt text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    /// Description of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the image should appear pixelated at sizes larger than the image's native resolution.
    #[serde(default)]
    pub pixelated: bool,
    /// Controls how the image is rendered. The value of this field supersedes the pixelated field.
    #[validate(enumerate = ["auto", "pixelated", "crisp-edges"])]
    #[serde(default = "default_auto")]
    pub image_rendering: String,
    /// Controls the appearance of the image. The "monochrome" value will mask the opaque parts of the image using the current text color.
    #[validate(enumerate = ["auto", "monochrome"])]
    #[serde(default = "default_auto")]
    pub appearance: String,
    /// Whether or not a background color is displayed behind the image.
    #[serde(default = "default_true")]
    pub background: bool,
    /// Whether or not the image is collapsed by default.
    #[serde(default)]
    pub collapsed: bool,
    /// Whether or not the image can be collapsed.
    #[serde(default = "default_true")]
    pub collapsible: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StructuredContentDefinition {
    /// Single definition for the term using a structured content object.
    pub content: Box<StructuredContent>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum StructuredContent {
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
    pub data: Option<StructuredContentData>,
}

/// Generic container tags.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct ContainerFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StructuredContentData>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// Table tags.
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TableElementFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StructuredContentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 1)]
    pub col_span: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 1)]
    pub row_span: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StructuredContentStyle>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// Container tags supporting configurable styles.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct StyledContainerFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StructuredContentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StructuredContentStyle>,
    /// Hover text for the element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether or not the details element is open by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// Image tag.
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImageFields {
    /// Path to the image file in the archive.
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<StructuredContentData>>,
    /// Preferred width of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    pub width: Option<f32>,
    /// Preferred height of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    pub height: Option<f32>,
    /// Hover text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Alt text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    /// Description of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the image should appear pixelated at sizes larger than the image's native resolution.
    #[serde(default)]
    pub pixelated: bool,
    /// Controls how the image is rendered. The value of this field supersedes the pixelated field.
    #[serde(default = "default_auto")]
    #[validate(enumerate = ["auto", "pixelated", "crisp-edges"])]
    pub image_rendering: String,
    /// Controls the appearance of the image. The "monochrome" value will mask the opaque parts of the image using the current text color.
    #[serde(default = "default_auto")]
    #[validate(enumerate = ["auto", "monochrome"])]
    pub appearance: String,
    /// Whether or not a background color is displayed behind the image.
    #[serde(default = "default_true")]
    pub background: bool,
    /// Whether or not the image is collapsed by default.
    #[serde(default)]
    pub collapsed: bool,
    /// Whether or not the image can be collapsed.
    #[serde(default)]
    pub collapsible: bool,
    /// The vertical alignment of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(enumerate = ["baseline", "sub", "super", "text-top", "text-bottom", "middle", "top", "bottom"])]
    pub vertical_align: Option<String>,
    /// Shorthand for border width, style, and color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    /// Roundness of the corners of the image's outer border edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    /// The units for the width and height.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(enumerate = ["px", "em"])]
    pub size_units: Option<String>,
}

/// Link tag.
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct LinkFields {
    #[validate(pattern = r"^(?:https?:|\?)[\w\W]*")]
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<StructuredContent>,
    /// Defines the language of an element in the format defined by RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// Generic data attributes that should be added to the element.
type StructuredContentData = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructuredContentStyle {
    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "italic"])]
    pub font_style: String,

    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "bold"])]
    pub font_weight: String,

    #[serde(default = "default_medium")]
    pub font_size: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    #[serde(default)]
    pub text_decoration_line: TextDecorationLine,

    #[serde(default = "default_solid")]
    #[validate(enumerate = ["solid", "double", "dotted", "dashed", "wavy"])]
    pub text_decoration_style: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_decoration_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_path: Option<String>,

    #[serde(default = "default_baseline")]
    #[validate(enumerate = ["baseline", "sub", "super", "text-top", "text-bottom", "middle", "top", "bottom"])]
    pub vertical_align: String,

    #[serde(default = "default_start")]
    #[validate(enumerate = ["start", "end", "left", "right", "center", "justify", "justify-all", "match-parent"])]
    pub text_align: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_emphasis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_shadow: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,

    #[serde(default)]
    pub margin_top: NumberOrString,

    #[serde(default)]
    pub margin_left: NumberOrString,

    #[serde(default)]
    pub margin_right: NumberOrString,

    #[serde(default)]
    pub margin_bottom: NumberOrString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_bottom: Option<String>,

    #[serde(default = "default_normal")]
    #[validate(enumerate = ["normal", "break-all", "keep-all"])]
    pub word_break: String,

    #[serde(default = "default_normal")]
    pub white_space: String,

    #[serde(default = "default_auto")]
    pub cursor: String,

    #[serde(default = "default_disc")]
    pub list_style_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum TextDecorationLine {
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
pub enum NumberOrString {
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
