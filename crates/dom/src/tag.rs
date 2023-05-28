use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    DocType,
    /// If the a element has an href attribute, then it represents a hyperlink (a hypertext anchor) labeled by its contents.
    A,
    /// The abbr element represents an abbreviation or acronym, optionally with its expansion. The title attribute may be used to provide an expansion of the abbreviation. The attribute, if specified, must contain an expansion of the abbreviation, and nothing else.
    Abbr,
    /// The acronym tag in HTML is used to define the acronym that gives useful information to browsers, translation systems, and search engines.
    Acronym,
    /// The address element represents the contact information for its nearest article or body element ancestor. If that is the body element, then the contact information applies to the document as a whole.
    Address,
    /// The `<altGlyph>` SVG element allows sophisticated selection of the glyphs used to render its child character data.
    AltGlyph,
    /// The `<altGlyphDef>` SVG element defines a substitution representation for glyphs.
    AltGlyphRef,
    /// The `<altGlyphItem>` element provides a set of candidates for glyph substitution by the `<altGlyph>` element.
    AltGlyphItem,
    /// The applet tag in HTML was used to embed Java applets into any HTML document, discontinued starting from HTML 5.
    Applet,
    /// The area element represents either a hyperlink with some text and a corresponding area on an image map, or a dead area on an image map.
    Area,
    /// The article element represents a complete, or self-contained, composition in a document, page, application, or site and that is, in principle, independently distributable or reusable, e.g. in syndication. This could be a forum post, a magazine or newspaper article, a blog entry, a user-submitted comment, an interactive widget or gadget, or any other independent item of content. Each article should be identified, typically by including a heading (h1–h6 element) as a child of the article element.
    Article,
    /// The aside element represents a section of a page that consists of content that is tangentially related to the content around the aside element, and which could be considered separate from that content. Such sections are often represented as sidebars in printed typography.
    Aside,
    /// An audio element represents a sound or audio stream.
    Audio,
    /// The b element represents a span of text to which attention is being drawn for utilitarian purposes without conveying any extra importance and with no implication of an alternate voice or mood, such as key words in a document abstract, product names in a review, actionable words in interactive text-driven software, or an article lede.
    B,
    /// The base element allows authors to specify the document base URL for the purposes of resolving relative URLs, and the name of the default browsing context for the purposes of following hyperlinks. The element does not represent any content beyond this information.
    Base,
    /// This tag is used to set the default text-color, font-size, & font-family of all the text in the browser. Not supported in HTML5.
    Basefont,
    /// The bdi tag refers to Bi-Directional Isolation. It differentiates a text from other text that may be formatted in a different direction.
    Bdi,
    /// The bdo element represents explicit text directionality formatting control for its children. It allows authors to override the Unicode bidirectional algorithm by explicitly specifying a direction override. [BIDI]
    Bdo,
    /// The bgsound tag is used to play the soundtrack in the background.
    Bgsound,
    /// The big tag in HTML is used to increase the selected text size by one larger than the surrounding text. In HTML 5.
    Big,
    /// The HTML blink tag is used to create a blinking text that flashes slowly. It has been obsolete all told fashionable browsers whereas some browsers never supported it in the least. This tag was also never standardized by hypertext mark-up language.
    Blink,
    /// The blockquote element represents content that is quoted from another source, optionally with a citation which must be within a footer or cite element, and optionally with in-line changes such as annotations and abbreviations.
    Blockquote,
    /// The body element represents the content of the document.
    Body,
    /// The br element represents a line break.
    Br,
    /// The button element represents a button labeled by its contents.
    Button,
    /// The caption element represents the title of the table that is its parent, if it has a parent and that is a table element.
    Caption,
    /// The canvas element provides scripts with a resolution-dependent bitmap canvas, which can be used for rendering graphs, game graphics, art, or other visual images on the fly.
    Canvas,
    /// The center tag in HTML is used to set the alignment of text in the center. Not supported in HTML5.
    Center,
    /// The cite element represents a reference to a creative work. It must include the title of the work or the name of the author(person, people or organization) or an URL reference, or a reference in abbreviated form as per the conventions used for the addition of citation metadata.
    Cite,
    /// The code element represents a fragment of computer code. This could be an XML element name, a file name, a computer program, or any other string that a computer would recognize.
    Code,
    /// The content HTML element—an obsolete part of the Web Components suite of technologies—was used inside of Shadow DOM as an insertion point, and wasn't meant to be used in ordinary HTML.
    Content,
    /// The colgroup element represents a group of one or more columns in the table that is its parent, if it has a parent and that is a table element.
    Colgroup,
    /// If a col element has a parent and that is a colgroup element that itself has a parent that is a table element, then the col element represents one or more columns in the column group represented by that colgroup.
    Col,
    Data,
    /// A datagrid represents an interactive representation of tree, list, or tabular data.
    /// The data being presented can come from one of the following:
    /// - Its own content (as elements given as children of the datagrid element). The <datagrid> element can contain other HTML elements. These elements are its children. The contents of its children is the data that is being presented.
    /// - From a scripted data provider given by the data DOM attribute.
    ///
    /// The <datagrid> tag was introduced in HTML 5.
    Datagrid,
    /// The datalist element represents a set of option elements that represent predefined options for other controls. In the rendering, the datalist element represents nothing and it, along with its children, should be hidden.
    Datalist,
    /// The dd element represents the description, definition, or value, part of a term-description group in a description list (dl element).
    Dd,
    /// The dfn element represents the defining instance of a term. The paragraph, description list group, or section that is the nearest ancestor of the dfn element must also contain the definition(s) for the term given by the dfn element.
    Dfn,
    /// The defs element is used to store graphical objects that will be used at a later time. Objects created inside a defs element are not rendered directly. To display them you have to reference them (with a use element for example).
    Defs,
    /// The del element represents a removal from the document.
    Del,
    /// The details element represents a disclosure widget from which the user can obtain additional information or controls.
    Details,
    /// This tag is used to create a popup dialog and models on a web page. This tag is new in HTML5.
    Dialog,
    /// The dir tag is used to make a list of directory titles. It is not supported in HTML 5 <ul> or CSS are used instead of <dir> tag.
    Dir,
    /// The div element has no special meaning at all. It represents its children. It can be used with the class, lang, and title attributes to mark up semantics common to a group of consecutive elements.
    Div,
    /// The dl element represents an association list consisting of zero or more name-value groups (a description list). A name-value group consists of one or more names (dt elements) followed by one or more values (dd elements), ignoring any nodes other than dt and dd elements. Within a single dl element, there should not be more than one dt element for each name.
    Dl,
    /// The dt element represents the term, or name, part of a term-description group in a description list (dl element).
    Dt,
    /// The em element represents stress emphasis of its contents.
    Em,
    /// The embed element provides an integration point for an external (typically non-HTML) application or interactive content.
    Embed,
    /// The fieldset element represents a set of form controls optionally grouped under a common name.
    Fieldset,
    /// The figcaption element represents a caption or legend for the rest of the contents of the figcaption element's parent figure element, if any.
    Figcaption,
    /// The figure element represents some flow content, optionally with a caption, that is self-contained (like a complete sentence) and is typically referenced as a single unit from the main flow of the document.
    Figure,
    /// The font tag in HTML plays an important role in the web page to create an attractive and readable web page.
    Font,
    /// The footer element represents a footer for its nearest ancestor sectioning content or sectioning root element. A footer typically contains information about its section such as who wrote it, links to related documents, copyright data, and the like.
    Footer,
    /// The form element represents a collection of form-associated elements, some of which can represent editable values that can be submitted to a server for processing.
    Form,
    /// HTML Frames are used to divide the web browser window into multiple sections. Not supported in HTML5.
    Frame,
    /// The frameset element contains one or more frame elements. It is used to specify the number of rows and columns in a frameset with their pixel of spaces.
    Frameset,
    /// The head element represents a collection of metadata for the Document.
    Head,
    /// The header element represents introductory content for its nearest ancestor sectioning content or sectioning root element. A header typically contains a group of introductory or navigational aids. When the nearest ancestor sectioning content or sectioning root element is the body element, then it applies to the whole page.
    Header,
    /// The h1 element represents a section heading.
    H1,
    /// The h2 element represents a section heading.
    H2,
    /// The h3 element represents a section heading.
    H3,
    /// The h4 element represents a section heading.
    H4,
    /// The h5 element represents a section heading.
    H5,
    /// The h6 element represents a section heading.
    H6,
    /// The hgroup tag in HTML is used to wrap one or more heading elements from <h1> to <h6>, such as the headings and sub-headings.
    Hgroup,
    /// The hr element represents a paragraph-level thematic break, e.g. a scene change in a story, or a transition to another topic within a section of a reference book.
    Hr,
    /// The html element represents the root of an HTML document.
    Html,
    /// The iframe element represents a nested browsing context.
    Iframe,
    /// The <image> HTML element is an ancient and poorly supported precursor to the <img> element. It should not be used.
    Image,
    /// An img element represents an image.
    Img,
    /// The input element represents a typed data field, usually with a form control to allow the user to edit the data.
    Input,
    /// The ins element represents an addition to the document.
    Ins,
    /// The index tag is used to query any document through a text field.
    Isindex,
    /// The i element represents a span of text in an alternate voice or mood, or otherwise offset from the normal prose in a manner indicating a different quality of text, such as a taxonomic designation, a technical term, an idiomatic phrase from another language, transliteration, a thought, or a ship name in Western texts.
    I,
    /// The kbd element represents user input (typically keyboard input, although it may also be used to represent other input, such as voice commands).
    Kbd,
    /// The keygen tag in HTML is used to specify a key-pair generator field in a form. When a form is submitted then two keys are generated, the private key and a public key.
    Keygen,
    /// The label element represents a caption in a user interface. The caption can be associated with a specific form control, known as the label element's labeled control, either using the for attribute, or by putting the form control inside the label element itself.
    Label,
    /// The legend element represents a caption for the rest of the contents of the legend element's parent fieldset element, if any.
    Legend,
    /// The li element represents a list item. If its parent element is an ol, ul, or menu element, then the element is an item of the parent element's list, as defined for those elements. Otherwise, the list item has no defined list-related relationship to any other li element.
    Li,
    /// The link element allows authors to link their document to other resources.
    Link,
    /// The main element represents the main content of the body of a document or application. The main content area consists of content that is directly related to or expands upon the central topic of a document or central functionality of an application.
    Main,
    /// The map element, in conjunction with an img element and any area element descendants, defines an image map. The element represents its children.
    Map,
    /// The mark tag in HTML is used to define the marked text. It is used to highlight the part of the text in a paragraph.
    Mark,
    /// The marquee tag in HTML is used to create scrolling text or images on a webpage. It scrolls either horizontally or vertically.
    Marquee,
    Menu,
    /// The menuitem tag is used to define a command or menu that the user can utilize from the popup item. Not supported in HTML5.
    Menuitem,
    /// The meta element represents various kinds of metadata that cannot be expressed using the title, base, link, style, and script elements.
    Meta,
    /// The meter element represents a scalar measurement within a known range, or a fractional value; for example disk usage, the relevance of a query result, or the fraction of a voting population to have selected a particular candidate.
    Meter,
    /// The nav element represents a section of a page that links to other pages or to parts within the page: a section with navigation links.
    Nav,
    /// The no break tag is used to create a single line text, that does not matter how long the statement is, this tag is used with <wbr> tag
    Nobr,
    /// The noembed tag is used to show that the browser is not supported by <embed> tag.
    Noembed,
    /// The `<noframes>` tag is the backup for those browsers that does not support frames. This tag can contains all the element that can be placed in <body> tag. It is used to create link with the non-frame set version of any website where you want to display a message to the user. This <noframes> tag is not supported in HTML5.
    Noframes,
    /// The noscript element represents nothing if scripting is enabled, and represents its children if scripting is disabled. It is used to present different markup to user agents that support scripting and those that don't support scripting, by affecting how the document is parsed.
    Noscript,
    /// The object element can represent an external resource, which, depending on the type of the resource, will either be treated as an image, as a nested browsing context, or as an external resource to be processed by a plugin.
    Object,
    /// The ol element represents a list of items, where the items have been intentionally ordered, such that changing the order would change the meaning of the document.
    Ol,
    /// The optgroup element represents a group of option elements with a common label.
    Optgroup,
    /// The option element represents an option in a select element or as part of a list of suggestions in a datalist element.
    Option,
    /// The output element represents the result of a calculation performed by the application, or the result of a user action.
    Output,
    /// The p element represents a paragraph.
    P,
    /// The param element defines parameters for plugins invoked by object elements. It does not represent anything on its own.
    Param,
    Picture,
    ///  The plaintext tag is used to display all the text in the document as it was initially typed in. Or in other words, this tag ignores all the formatting and displays all the text present below this tag including tag and document tags. This tag, can’t be turned off, and can’t be stopped. Everything below the <plaintext> tag is shown below as it is, it does not have a closing tag or we can say that it is an empty tag.
    Plaintext,
    /// The pre element represents a block of preformatted text, in which structure is represented by typographic conventions rather than by elements.
    Pre,
    /// The progress element represents the completion progress of a task. The progress is either indeterminate, indicating that progress is being made but that it is not clear how much more work remains to be done before the task is complete (e.g. because the task is waiting for a remote host to respond), or the progress is a number in the range zero to a maximum, giving the fraction of work that has so far been completed.
    Progress,
    /// The q element represents some phrasing content quoted from another source.
    Q,
    /// The rb HTML element is used to delimit the base text component of a ruby annotation, i.e. the text that is being annotated. One rb element should wrap each separate atomic segment of the base text.
    Rb,
    /// The rp element is used to provide fallback text to be shown by user agents that don't support ruby annotations. One widespread convention is to provide parentheses around the ruby text component of a ruby annotation.
    Rp,
    /// The rt element marks the ruby text component of a ruby annotation. When it is the child of a ruby element or of an rtc element that is itself the child of a ruby element, it doesn't represent anything itself, but its ancestor ruby element uses it as part of determining what it represents.
    Rt,
    /// The rtc HTML element embraces semantic annotations of characters presented in a ruby of rb elements used inside of ruby element. rb elements can have both pronunciation (rt) and semantic (rtc) annotations.
    Rtc,
    /// The ruby element allows one or more spans of phrasing content to be marked with ruby annotations. Ruby annotations are short runs of text presented alongside base text, primarily used in East Asian typography as a guide for pronunciation or to include other annotations. In Japanese, this form of typography is also known as furigana. Ruby text can appear on either side, and sometimes both sides, of the base text, and it is possible to control its position using CSS. A more complete introduction to ruby can be found in the Use Cases & Exploratory Approaches for Ruby Markup document as well as in CSS Ruby Module Level 1. [RUBY-UC] [CSSRUBY]
    Ruby,
    /// The s element represents contents that are no longer accurate or no longer relevant.
    S,
    /// The samp element represents sample or quoted output from another program or computing system.
    Samp,
    /// The script element allows authors to include dynamic script and data blocks in their documents. The element does not represent content for the user.
    Script,
    /// The section element represents a generic section of a document or application. A section, in this context, is a thematic grouping of content. Each section should be identified, typically by including a heading ( h1- h6 element) as a child of the section element.
    Section,
    /// The select element represents a control for selecting amongst a set of options.
    Select,
    /// The `<shadow>` HTML element—an obsolete part of the Web Components technology suite—was intended to be used as a shadow DOM insertion point. You might have used it if you have created multiple shadow roots under a shadow host. It is not useful in ordinary HTML.
    Shadow,
    Slot,
    /// The small element represents side comments such as small print.
    Small,
    /// The source element allows authors to specify multiple alternative media resources for media elements. It does not represent anything on its own.
    Source,
    /// The spacer tag is used to create some white space. Not-supported in HTML5 .
    Spacer,
    /// The span element doesn't mean anything on its own, but can be useful when used together with the global attributes, e.g. class, lang, or dir. It represents its children.
    Span,
    /// HTML strike tag, along with understanding its implementation through the example. The strike tag defines a strike or line through Text.
    Strike,
    /// The strong element represents strong importance, seriousness, or urgency for its contents.
    Strong,
    /// The style element allows authors to embed style information in their documents. The style element is one of several inputs to the styling processing model. The element does not represent content for the user.
    Style,
    /// The sub element represents a subscript.
    Sub,
    /// The sup element represents a superscript.
    Sup,
    /// The summary element represents a summary, caption, or legend for the rest of the contents of the summary element's parent details element, if any.
    Summary,
    /// The svg element can be used to embed an SVG fragment inside the current document (for example, an HTML document). This SVG fragment has its own viewport and coordinate system.
    Svg,
    /// The table element represents data with more than one dimension, in the form of a table.
    Table,
    /// The tbody element represents a block of rows that consist of a body of data for the parent table element, if the tbody element has a parent and it is a table.
    Tbody,
    /// The td element represents a data cell in a table.
    Td,
    /// The template element is used to declare fragments of HTML that can be cloned and inserted in the document by script.
    Template,
    /// The textarea element represents a multiline plain text edit control for the element's raw value. The contents of the control represent the control's default value.
    Textarea,
    /// The tfoot element represents the block of rows that consist of the column summaries (footers) for the parent table element, if the tfoot element has a parent and it is a table.
    Tfoot,
    /// The th element represents a header cell in a table.
    Th,
    /// The thead element represents the block of rows that consist of the column labels (headers) for the parent table element, if the thead element has a parent and it is a table.
    Thead,
    /// The time element represents its contents, along with a machine-readable form of those contents in the datetime attribute. The kind of content is limited to various kinds of dates, times, time-zone offsets, and durations.
    Time,
    /// The title element represents the document's title or name. Authors should use titles that identify their documents even when they are used out of context, for example in a user's history or bookmarks, or in search results. The document's title is often different from its first heading, since the first heading does not have to stand alone when taken out of context.
    Title,
    /// The tr element represents a row of cells in a table.
    Tr,
    /// The track element allows authors to specify explicit external timed text tracks for media elements. It does not represent anything on its own.
    Track,
    /// The tt tag is the abbreviation of teletype text. This tag is depreciated from HTML 5. It was used for marking Keyboard input.
    Tt,
    /// The u element represents a span of text with an unarticulated, though explicitly rendered, non-textual annotation, such as labeling the text as being a proper name in Chinese text (a Chinese proper name mark), or labeling the text as being misspelt.
    U,
    /// The ul element represents a list of items, where the order of the items is not important — that is, where changing the order would not materially change the meaning of the document.
    Ul,
    /// The use element takes nodes from within the SVG document, and duplicates them somewhere else. The effect is the same as if the nodes were deeply cloned into a non-exposed DOM, then pasted where the use element is, much like cloned template elements.
    Use,
    /// The var element represents a variable. This could be an actual variable in a mathematical expression or programming context, an identifier representing a constant, a symbol identifying a physical quantity, a function parameter, or just be a term used as a placeholder in prose.
    Var,
    /// A video element is used for playing videos or movies, and audio files with captions.
    Video,
    /// The wbr element represents a line break opportunity.
    Wbr,
    Xmp,
    /// Tuple of index of the first charcter in the tag in its origin document, and length of the unknown tag.
    Unknown(String),
}

impl Tag {
    /// Check if the tag is deprecated.
    pub fn is_deprecated(&self) -> bool {
        match self {
            Self::Acronym
            | Self::AltGlyph
            | Self::Applet
            | Self::Big
            | Self::Basefont
            | Self::Bgsound
            | Self::Blink
            | Self::Content
            | Self::Dir
            | Self::Frame
            | Self::Font
            | Self::Frameset
            | Self::Isindex
            | Self::Image
            | Self::Marquee
            | Self::Menuitem
            | Self::Param
            | Self::Rb
            | Self::Rtc
            | Self::Strike
            | Self::Shadow
            | Self::Center
            | Self::Plaintext
            | Self::Noframes
            | Self::Nobr
            | Self::Noembed
            | Self::Spacer
            | Self::Tt
            | Self::Xmp => true,
            _ => false,
        }
    }
    /// Check if the tag can be void.
    pub fn is_void(&self) -> bool {
        match self {
            Self::Area
            | Self::Base
            | Self::Br
            | Self::Col
            | Self::Embed
            | Self::Hr
            | Self::Img
            | Self::Input
            | Self::Keygen
            | Self::Link
            | Self::Meta
            | Self::Param
            | Self::Source
            | Self::Track
            | Self::Wbr => true,
            _ => false,
        }
    }
    /// Convert the tag to an uppercase string slice.
    pub fn as_str(&self) -> &str {
        todo!()
    }
    /// Check if the tag is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Tag::Unknown(_))
    }
}

impl From<&str> for Tag {
    fn from(input: &str) -> Self {
        match input {
            "!doctype" | "!DOCTYPE" => Self::DocType,
            "a" => Self::A,
            "abbr" => Self::Abbr,
            "acronym" => Self::Acronym,
            "address" => Self::Address,
            "applet" => Self::Applet,
            "area" => Self::Area,
            "article" => Self::Article,
            "aside" => Self::Aside,
            "audio" => Self::Audio,
            "b" => Self::B,
            "base" => Self::Base,
            "basefont" => Self::Basefont,
            "bdi" => Self::Bdi,
            "bdo" => Self::Bdo,
            "bgsound" => Self::Bgsound,
            "big" => Self::Big,
            "blink" => Self::Blink,
            "blockquote" => Self::Blockquote,
            "body" => Self::Body,
            "br" => Self::Br,
            "button" => Self::Button,
            "canvas" => Self::Canvas,
            "caption" => Self::Caption,
            "center" => Self::Center,
            "cite" => Self::Cite,
            "code" => Self::Code,
            "col" => Self::Col,
            "colgroup" => Self::Colgroup,
            "data" => Self::Data,
            "datalist" => Self::Datalist,
            "dd" => Self::Dd,
            "del" => Self::Del,
            "details" => Self::Details,
            "dfn" => Self::Dfn,
            "dialog" => Self::Dialog,
            "dir" => Self::Dir,
            "div" => Self::Div,
            "dl" => Self::Dl,
            "dt" => Self::Dt,
            "em" => Self::Em,
            "embed" => Self::Embed,
            "fieldset" => Self::Fieldset,
            "figcaption" => Self::Figcaption,
            "figure" => Self::Figure,
            "font" => Self::Font,
            "footer" => Self::Footer,
            "form" => Self::Form,
            "frame" => Self::Frame,
            "frameset" => Self::Frameset,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "h4" => Self::H4,
            "h5" => Self::H5,
            "h6" => Self::H6,
            "head" => Self::Head,
            "header" => Self::Header,
            "hr" => Self::Hr,
            "html" => Self::Html,
            "i" => Self::I,
            "iframe" => Self::Iframe,
            "image" => Self::Image,
            "img" => Self::Img,
            "input" => Self::Input,
            "ins" => Self::Ins,
            "isindex" => Self::Isindex,
            "kbd" => Self::Kbd,
            "keygen" => Self::Keygen,
            "label" => Self::Label,
            "legend" => Self::Legend,
            "li" => Self::Li,
            "link" => Self::Link,
            "main" => Self::Main,
            "map" => Self::Map,
            "mark" => Self::Mark,
            "marquee" => Self::Marquee,
            "menu" => Self::Menu,
            "menuitem" => Self::Menuitem,
            "meta" => Self::Meta,
            "meter" => Self::Meter,
            "nav" => Self::Nav,
            "nobr" => Self::Nobr,
            "noembed" => Self::Noembed,
            "noframes" => Self::Noframes,
            "noscript" => Self::Noscript,
            "object" => Self::Object,
            "ol" => Self::Ol,
            "optgroup" => Self::Optgroup,
            "option" => Self::Option,
            "output" => Self::Output,
            "p" => Self::P,
            "param" => Self::Param,
            "picture" => Self::Picture,
            "plaintext" => Self::Plaintext,
            "pre" => Self::Pre,
            "progress" => Self::Progress,
            "q" => Self::Q,
            "rp" => Self::Rp,
            "rt" => Self::Rt,
            "ruby" => Self::Ruby,
            "s" => Self::S,
            "samp" => Self::Samp,
            "script" => Self::Script,
            "section" => Self::Section,
            "select" => Self::Select,
            "small" => Self::Small,
            "slot" => Self::Slot,
            "source" => Self::Source,
            "spacer" => Self::Spacer,
            "span" => Self::Span,
            "strike" => Self::Strike,
            "strong" => Self::Strong,
            "style" => Self::Style,
            "sub" => Self::Sub,
            "summary" => Self::Summary,
            "sup" => Self::Sup,
            "svg" => Self::Svg,
            "table" => Self::Table,
            "tbody" => Self::Tbody,
            "td" => Self::Td,
            "template" => Self::Template,
            "textarea" => Self::Textarea,
            "tfoot" => Self::Tfoot,
            "th" => Self::Th,
            "thead" => Self::Thead,
            "time" => Self::Time,
            "title" => Self::Title,
            "tr" => Self::Tr,
            "track" => Self::Track,
            "tt" => Self::Tt,
            "u" => Self::U,
            "ul" => Self::Ul,
            "var" => Self::Var,
            "video" => Self::Video,
            "wbr" => Self::Wbr,
            "xmp" => Self::Xmp,
            _ => Tag::Unknown(input.to_owned()),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tag::DocType => "doctype",
                Tag::A => "a",
                Tag::Abbr => "abbr",
                Tag::Acronym => "acronym",
                Tag::Address => "address",
                Tag::AltGlyph => "altGlyph",
                Tag::AltGlyphRef => "altGlyphRef",
                Tag::AltGlyphItem => "altGlyphItem",
                Tag::Applet => "applet",
                Tag::Area => "area",
                Tag::Article => "article",
                Tag::Aside => "aside",
                Tag::Audio => "audio",
                Tag::B => "b",
                Tag::Base => "base",
                Tag::Basefont => "basefont",
                Tag::Bdi => "bdi",
                Tag::Bdo => "bdo",
                Tag::Bgsound => "bgsound",
                Tag::Big => "big",
                Tag::Blink => "blink",
                Tag::Blockquote => "blockquote",
                Tag::Body => "body",
                Tag::Br => "br",
                Tag::Button => "button",
                Tag::Caption => "caption",
                Tag::Canvas => "canvas",
                Tag::Center => "center",
                Tag::Cite => "cite",
                Tag::Code => "code",
                Tag::Content => "content",
                Tag::Colgroup => "colgroup",
                Tag::Col => "col",
                Tag::Data => "data",
                Tag::Datagrid => "datagrid",
                Tag::Datalist => "datalist",
                Tag::Dd => "dd",
                Tag::Dfn => "dfn",
                Tag::Defs => "defs",
                Tag::Del => "del",
                Tag::Details => "details",
                Tag::Dialog => "dialog",
                Tag::Dir => "dir",
                Tag::Div => "div",
                Tag::Dl => "dl",
                Tag::Dt => "dt",
                Tag::Em => "em",
                Tag::Embed => "embed",
                Tag::Fieldset => "fieldset",
                Tag::Figcaption => "figcaption",
                Tag::Figure => "figure",
                Tag::Font => "font",
                Tag::Footer => "footer",
                Tag::Form => "form",
                Tag::Frame => "frame",
                Tag::Frameset => "frameset",
                Tag::Head => "head",
                Tag::Header => "header",
                Tag::H1 => "h1",
                Tag::H2 => "h2",
                Tag::H3 => "h3",
                Tag::H4 => "h4",
                Tag::H5 => "h5",
                Tag::H6 => "h6",
                Tag::Hgroup => "hgroup",
                Tag::Hr => "hr",
                Tag::Html => "html",
                Tag::Iframe => "iframe",
                Tag::Image => "image",
                Tag::Img => "img",
                Tag::Input => "input",
                Tag::Ins => "ins",
                Tag::Isindex => "isindex",
                Tag::I => "i",
                Tag::Kbd => "kbd",
                Tag::Keygen => "keygen",
                Tag::Label => "label",
                Tag::Legend => "legend",
                Tag::Li => "li",
                Tag::Link => "link",
                Tag::Main => "main",
                Tag::Map => "map",
                Tag::Mark => "mark",
                Tag::Marquee => "marquee",
                Tag::Menu => "menu",
                Tag::Menuitem => "menuitem",
                Tag::Meta => "meta",
                Tag::Meter => "meter",
                Tag::Nav => "nav",
                Tag::Nobr => "nobr",
                Tag::Noembed => "noembed",
                Tag::Noframes => "noframes",
                Tag::Noscript => "noscript",
                Tag::Object => "object",
                Tag::Ol => "ol",
                Tag::Optgroup => "optgroup",
                Tag::Option => "option",
                Tag::Output => "output",
                Tag::P => "p",
                Tag::Param => "param",
                Tag::Picture => "picture",
                Tag::Plaintext => "plaintext",
                Tag::Pre => "pre",
                Tag::Progress => "progress",
                Tag::Q => "q",
                Tag::Rb => "rb",
                Tag::Rp => "rp",
                Tag::Rt => "rt",
                Tag::Rtc => "rtc",
                Tag::Ruby => "ruby",
                Tag::S => "s",
                Tag::Samp => "samp",
                Tag::Script => "script",
                Tag::Section => "section",
                Tag::Select => "select",
                Tag::Shadow => "shadow",
                Tag::Slot => "slot",
                Tag::Small => "small",
                Tag::Source => "source",
                Tag::Spacer => "spacer",
                Tag::Span => "span",
                Tag::Strike => "strike",
                Tag::Strong => "strong",
                Tag::Style => "style",
                Tag::Sub => "sub",
                Tag::Sup => "sup",
                Tag::Summary => "summary",
                Tag::Svg => "svg",
                Tag::Table => "table",
                Tag::Tbody => "tbody",
                Tag::Td => "td",
                Tag::Template => "template",
                Tag::Textarea => "textarea",
                Tag::Tfoot => "tfoot",
                Tag::Th => "th",
                Tag::Thead => "thead",
                Tag::Time => "time",
                Tag::Title => "title",
                Tag::Tr => "tr",
                Tag::Track => "track",
                Tag::Tt => "tt",
                Tag::U => "u",
                Tag::Ul => "ul",
                Tag::Use => "use",
                Tag::Var => "var",
                Tag::Video => "video",
                Tag::Wbr => "wbr",
                Tag::Xmp => "xmp",
                Tag::Unknown(s) => s,
            }
        )
    }
}
