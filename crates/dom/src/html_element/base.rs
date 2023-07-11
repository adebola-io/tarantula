use crate::{document::WeakDocumentRef, tag::Tag, Element};

#[derive(Debug)]
pub enum HTMLElementBase {
    DocType(Element),
    A(Element),
    Abbr(Element),
    Acronym(Element),
    Address(Element),
    Applet(Element),
    Area(Element),
    Article(Element),
    Aside(Element),
    Audio(Element),
    B(Element),
    Base(Element),
    Basefont(Element),
    Bdi(Element),
    Bdo(Element),
    Bgsound(Element),
    Big(Element),
    Blink(Element),
    Blockquote(Element),
    Body(Element),
    Br(Element),
    Button(Element),
    Canvas(Element),
    Caption(Element),
    Center(Element),
    Cite(Element),
    Code(Element),
    Col(Element),
    Colgroup(Element),
    Data(Element),
    Datalist(Element),
    Dd(Element),
    Del(Element),
    Details(Element),
    Dfn(Element),
    Dialog(Element),
    Dir(Element),
    Div(Element),
    Dl(Element),
    Dt(Element),
    Em(Element),
    Embed(Element),
    Fieldset(Element),
    Figcaption(Element),
    Figure(Element),
    Font(Element),
    Footer(Element),
    Form(Element),
    Frame(Element),
    Frameset(Element),
    H1(Element),
    H2(Element),
    H3(Element),
    H4(Element),
    H5(Element),
    H6(Element),
    Head(Element),
    Header(Element),
    Hr(Element),
    Html(Element),
    I(Element),
    Iframe(Element),
    Image(Element),
    Img(Element),
    Input { value: String, element: Element },
    Ins(Element),
    Isindex(Element),
    Kbd(Element),
    Keygen(Element),
    Label(Element),
    Legend(Element),
    Li(Element),
    Link(Element),
    Main(Element),
    Map(Element),
    Mark(Element),
    Marquee(Element),
    Menu(Element),
    Menuitem(Element),
    Meta(Element),
    Meter(Element),
    Nav(Element),
    Nobr(Element),
    Noembed(Element),
    Noframes(Element),
    Noscript(Element),
    Object(Element),
    Ol(Element),
    Optgroup(Element),
    Option(Element),
    Output(Element),
    P(Element),
    Param(Element),
    Picture(Element),
    Plaintext(Element),
    Pre(Element),
    Progress(Element),
    Q(Element),
    Rp(Element),
    Rt(Element),
    Ruby(Element),
    S(Element),
    Samp(Element),
    Script(Element),
    Section(Element),
    Select(Element),
    Small(Element),
    Slot(Element),
    Source(Element),
    Spacer(Element),
    Span(Element),
    Strike(Element),
    Strong(Element),
    Style(Element),
    Sub(Element),
    Summary(Element),
    Sup(Element),
    Svg(Element),
    Table(Element),
    Tbody(Element),
    Td(Element),
    Template(Element),
    Textarea(Element),
    Tfoot(Element),
    Th(Element),
    Thead(Element),
    Time(Element),
    Title(Element),
    Tr(Element),
    Track(Element),
    Tt(Element),
    U(Element),
    Ul(Element),
    Var(Element),
    Video(Element),
    Wbr(Element),
    Xmp(Element),
    Unknown(Element),
}
impl HTMLElementBase {
    pub(crate) fn create(tagname: &str, weak_ref: WeakDocumentRef) -> Self {
        let is_html = true;
        let elem_create = Element::in_document;
        match tagname {
            "!doctype" | "!DOCTYPE" => Self::DocType(elem_create(Tag::DocType, is_html, weak_ref)),
            "a" => Self::A(elem_create(Tag::A, is_html, weak_ref)),
            "abbr" => Self::Abbr(elem_create(Tag::Abbr, is_html, weak_ref)),
            "acronym" => Self::Acronym(elem_create(Tag::Acronym, is_html, weak_ref)),
            "address" => Self::Address(elem_create(Tag::Address, is_html, weak_ref)),
            "applet" => Self::Applet(elem_create(Tag::Applet, is_html, weak_ref)),
            "area" => Self::Area(elem_create(Tag::Area, is_html, weak_ref)),
            "article" => Self::Article(elem_create(Tag::Article, is_html, weak_ref)),
            "aside" => Self::Aside(elem_create(Tag::Aside, is_html, weak_ref)),
            "audio" => Self::Audio(elem_create(Tag::Audio, is_html, weak_ref)),
            "b" => Self::B(elem_create(Tag::B, is_html, weak_ref)),
            "base" => Self::Base(elem_create(Tag::Base, is_html, weak_ref)),
            "basefont" => Self::Basefont(elem_create(Tag::Basefont, is_html, weak_ref)),
            "bdi" => Self::Bdi(elem_create(Tag::Bdi, is_html, weak_ref)),
            "bdo" => Self::Bdo(elem_create(Tag::Bdo, is_html, weak_ref)),
            "bgsound" => Self::Bgsound(elem_create(Tag::Bgsound, is_html, weak_ref)),
            "big" => Self::Big(elem_create(Tag::Big, is_html, weak_ref)),
            "blink" => Self::Blink(elem_create(Tag::Blink, is_html, weak_ref)),
            "blockquote" => Self::Blockquote(elem_create(Tag::Blockquote, is_html, weak_ref)),
            "body" => Self::Body(elem_create(Tag::Body, is_html, weak_ref)),
            "br" => Self::Br(elem_create(Tag::Br, is_html, weak_ref)),
            "button" => Self::Button(elem_create(Tag::Button, is_html, weak_ref)),
            "canvas" => Self::Canvas(elem_create(Tag::Canvas, is_html, weak_ref)),
            "caption" => Self::Caption(elem_create(Tag::Caption, is_html, weak_ref)),
            "center" => Self::Center(elem_create(Tag::Center, is_html, weak_ref)),
            "cite" => Self::Cite(elem_create(Tag::Cite, is_html, weak_ref)),
            "code" => Self::Code(elem_create(Tag::Code, is_html, weak_ref)),
            "col" => Self::Col(elem_create(Tag::Col, is_html, weak_ref)),
            "colgroup" => Self::Colgroup(elem_create(Tag::Colgroup, is_html, weak_ref)),
            "data" => Self::Data(elem_create(Tag::Data, is_html, weak_ref)),
            "datalist" => Self::Datalist(elem_create(Tag::Datalist, is_html, weak_ref)),
            "dd" => Self::Dd(elem_create(Tag::Dd, is_html, weak_ref)),
            "del" => Self::Del(elem_create(Tag::Del, is_html, weak_ref)),
            "details" => Self::Details(elem_create(Tag::Details, is_html, weak_ref)),
            "dfn" => Self::Dfn(elem_create(Tag::Dfn, is_html, weak_ref)),
            "dialog" => Self::Dialog(elem_create(Tag::Dialog, is_html, weak_ref)),
            "dir" => Self::Dir(elem_create(Tag::Dir, is_html, weak_ref)),
            "div" => Self::Div(elem_create(Tag::Div, is_html, weak_ref)),
            "dl" => Self::Dl(elem_create(Tag::Dl, is_html, weak_ref)),
            "dt" => Self::Dt(elem_create(Tag::Dt, is_html, weak_ref)),
            "em" => Self::Em(elem_create(Tag::Em, is_html, weak_ref)),
            "embed" => Self::Embed(elem_create(Tag::Embed, is_html, weak_ref)),
            "fieldset" => Self::Fieldset(elem_create(Tag::Fieldset, is_html, weak_ref)),
            "figcaption" => Self::Figcaption(elem_create(Tag::Figcaption, is_html, weak_ref)),
            "figure" => Self::Figure(elem_create(Tag::Figure, is_html, weak_ref)),
            "font" => Self::Font(elem_create(Tag::Font, is_html, weak_ref)),
            "footer" => Self::Footer(elem_create(Tag::Footer, is_html, weak_ref)),
            "form" => Self::Form(elem_create(Tag::Form, is_html, weak_ref)),
            "frame" => Self::Frame(elem_create(Tag::Frame, is_html, weak_ref)),
            "frameset" => Self::Frameset(elem_create(Tag::Frameset, is_html, weak_ref)),
            "h1" => Self::H1(elem_create(Tag::H1, is_html, weak_ref)),
            "h2" => Self::H2(elem_create(Tag::H2, is_html, weak_ref)),
            "h3" => Self::H3(elem_create(Tag::H3, is_html, weak_ref)),
            "h4" => Self::H4(elem_create(Tag::H4, is_html, weak_ref)),
            "h5" => Self::H5(elem_create(Tag::H5, is_html, weak_ref)),
            "h6" => Self::H6(elem_create(Tag::H6, is_html, weak_ref)),
            "head" => Self::Head(elem_create(Tag::Head, is_html, weak_ref)),
            "header" => Self::Header(elem_create(Tag::Header, is_html, weak_ref)),
            "hr" => Self::Hr(elem_create(Tag::Hr, is_html, weak_ref)),
            "html" => Self::Html(elem_create(Tag::Html, is_html, weak_ref)),
            "i" => Self::I(elem_create(Tag::I, is_html, weak_ref)),
            "iframe" => Self::Iframe(elem_create(Tag::Iframe, is_html, weak_ref)),
            "image" => Self::Image(elem_create(Tag::Image, is_html, weak_ref)),
            "img" => Self::Img(elem_create(Tag::Img, is_html, weak_ref)),
            "input" => Self::Input {
                element: elem_create(Tag::Input, is_html, weak_ref),
                value: String::new(),
            },
            "ins" => Self::Ins(elem_create(Tag::Ins, is_html, weak_ref)),
            "isindex" => Self::Isindex(elem_create(Tag::Isindex, is_html, weak_ref)),
            "kbd" => Self::Kbd(elem_create(Tag::Kbd, is_html, weak_ref)),
            "keygen" => Self::Keygen(elem_create(Tag::Keygen, is_html, weak_ref)),
            "label" => Self::Label(elem_create(Tag::Label, is_html, weak_ref)),
            "legend" => Self::Legend(elem_create(Tag::Legend, is_html, weak_ref)),
            "li" => Self::Li(elem_create(Tag::Li, is_html, weak_ref)),
            "link" => Self::Link(elem_create(Tag::Link, is_html, weak_ref)),
            "main" => Self::Main(elem_create(Tag::Main, is_html, weak_ref)),
            "map" => Self::Map(elem_create(Tag::Map, is_html, weak_ref)),
            "mark" => Self::Mark(elem_create(Tag::Mark, is_html, weak_ref)),
            "marquee" => Self::Marquee(elem_create(Tag::Marquee, is_html, weak_ref)),
            "menu" => Self::Menu(elem_create(Tag::Menu, is_html, weak_ref)),
            "menuitem" => Self::Menuitem(elem_create(Tag::Menuitem, is_html, weak_ref)),
            "meta" => Self::Meta(elem_create(Tag::Meta, is_html, weak_ref)),
            "meter" => Self::Meter(elem_create(Tag::Meter, is_html, weak_ref)),
            "nav" => Self::Nav(elem_create(Tag::Nav, is_html, weak_ref)),
            "nobr" => Self::Nobr(elem_create(Tag::Nobr, is_html, weak_ref)),
            "noembed" => Self::Noembed(elem_create(Tag::Noembed, is_html, weak_ref)),
            "noframes" => Self::Noframes(elem_create(Tag::Noframes, is_html, weak_ref)),
            "noscript" => Self::Noscript(elem_create(Tag::Noscript, is_html, weak_ref)),
            "object" => Self::Object(elem_create(Tag::Object, is_html, weak_ref)),
            "ol" => Self::Ol(elem_create(Tag::Ol, is_html, weak_ref)),
            "optgroup" => Self::Optgroup(elem_create(Tag::Optgroup, is_html, weak_ref)),
            "option" => Self::Option(elem_create(Tag::Option, is_html, weak_ref)),
            "output" => Self::Output(elem_create(Tag::Output, is_html, weak_ref)),
            "p" => Self::P(elem_create(Tag::P, is_html, weak_ref)),
            "param" => Self::Param(elem_create(Tag::Param, is_html, weak_ref)),
            "picture" => Self::Picture(elem_create(Tag::Picture, is_html, weak_ref)),
            "plaintext" => Self::Plaintext(elem_create(Tag::Plaintext, is_html, weak_ref)),
            "pre" => Self::Pre(elem_create(Tag::Pre, is_html, weak_ref)),
            "progress" => Self::Progress(elem_create(Tag::Progress, is_html, weak_ref)),
            "q" => Self::Q(elem_create(Tag::Q, is_html, weak_ref)),
            "rp" => Self::Rp(elem_create(Tag::Rp, is_html, weak_ref)),
            "rt" => Self::Rt(elem_create(Tag::Rt, is_html, weak_ref)),
            "ruby" => Self::Ruby(elem_create(Tag::Ruby, is_html, weak_ref)),
            "s" => Self::S(elem_create(Tag::S, is_html, weak_ref)),
            "samp" => Self::Samp(elem_create(Tag::Samp, is_html, weak_ref)),
            "script" => Self::Script(elem_create(Tag::Script, is_html, weak_ref)),
            "section" => Self::Section(elem_create(Tag::Section, is_html, weak_ref)),
            "select" => Self::Select(elem_create(Tag::Select, is_html, weak_ref)),
            "small" => Self::Small(elem_create(Tag::Small, is_html, weak_ref)),
            "slot" => Self::Slot(elem_create(Tag::Slot, is_html, weak_ref)),
            "source" => Self::Source(elem_create(Tag::Source, is_html, weak_ref)),
            "spacer" => Self::Spacer(elem_create(Tag::Spacer, is_html, weak_ref)),
            "span" => Self::Span(elem_create(Tag::Span, is_html, weak_ref)),
            "strike" => Self::Strike(elem_create(Tag::Strike, is_html, weak_ref)),
            "strong" => Self::Strong(elem_create(Tag::Strong, is_html, weak_ref)),
            "style" => Self::Style(elem_create(Tag::Style, is_html, weak_ref)),
            "sub" => Self::Sub(elem_create(Tag::Sub, is_html, weak_ref)),
            "summary" => Self::Summary(elem_create(Tag::Summary, is_html, weak_ref)),
            "sup" => Self::Sup(elem_create(Tag::Sup, is_html, weak_ref)),
            "svg" => Self::Svg(elem_create(Tag::Svg, is_html, weak_ref)),
            "table" => Self::Table(elem_create(Tag::Table, is_html, weak_ref)),
            "tbody" => Self::Tbody(elem_create(Tag::Tbody, is_html, weak_ref)),
            "td" => Self::Td(elem_create(Tag::Td, is_html, weak_ref)),
            "template" => Self::Template(elem_create(Tag::Template, is_html, weak_ref)),
            "textarea" => Self::Textarea(elem_create(Tag::Textarea, is_html, weak_ref)),
            "tfoot" => Self::Tfoot(elem_create(Tag::Tfoot, is_html, weak_ref)),
            "th" => Self::Th(elem_create(Tag::Th, is_html, weak_ref)),
            "thead" => Self::Thead(elem_create(Tag::Thead, is_html, weak_ref)),
            "time" => Self::Time(elem_create(Tag::Time, is_html, weak_ref)),
            "title" => Self::Title(elem_create(Tag::Title, is_html, weak_ref)),
            "tr" => Self::Tr(elem_create(Tag::Tr, is_html, weak_ref)),
            "track" => Self::Track(elem_create(Tag::Track, is_html, weak_ref)),
            "tt" => Self::Tt(elem_create(Tag::Tt, is_html, weak_ref)),
            "u" => Self::U(elem_create(Tag::U, is_html, weak_ref)),
            "ul" => Self::Ul(elem_create(Tag::Ul, is_html, weak_ref)),
            "var" => Self::Var(elem_create(Tag::Var, is_html, weak_ref)),
            "video" => Self::Video(elem_create(Tag::Video, is_html, weak_ref)),
            "wbr" => Self::Wbr(elem_create(Tag::Wbr, is_html, weak_ref)),
            "xmp" => Self::Xmp(elem_create(Tag::Xmp, is_html, weak_ref)),
            _ => Self::Unknown(elem_create(
                Tag::Unknown(tagname.to_owned()),
                is_html,
                weak_ref,
            )),
        }
    }

    pub(crate) fn element(&self) -> &Element {
        match self {
            Self::DocType(element)
            | Self::A(element)
            | Self::Abbr(element)
            | Self::Acronym(element)
            | Self::Address(element)
            | Self::Applet(element)
            | Self::Area(element)
            | Self::Article(element)
            | Self::Aside(element)
            | Self::Audio(element)
            | Self::B(element)
            | Self::Base(element)
            | Self::Basefont(element)
            | Self::Bdi(element)
            | Self::Bdo(element)
            | Self::Bgsound(element)
            | Self::Big(element)
            | Self::Blink(element)
            | Self::Blockquote(element)
            | Self::Body(element)
            | Self::Br(element)
            | Self::Button(element)
            | Self::Canvas(element)
            | Self::Caption(element)
            | Self::Center(element)
            | Self::Cite(element)
            | Self::Code(element)
            | Self::Col(element)
            | Self::Colgroup(element)
            | Self::Data(element)
            | Self::Datalist(element)
            | Self::Dd(element)
            | Self::Del(element)
            | Self::Details(element)
            | Self::Dfn(element)
            | Self::Dialog(element)
            | Self::Dir(element)
            | Self::Div(element)
            | Self::Dl(element)
            | Self::Dt(element)
            | Self::Em(element)
            | Self::Embed(element)
            | Self::Fieldset(element)
            | Self::Figcaption(element)
            | Self::Figure(element)
            | Self::Font(element)
            | Self::Footer(element)
            | Self::Form(element)
            | Self::Frame(element)
            | Self::Frameset(element)
            | Self::H1(element)
            | Self::H2(element)
            | Self::H3(element)
            | Self::H4(element)
            | Self::H5(element)
            | Self::H6(element)
            | Self::Head(element)
            | Self::Header(element)
            | Self::Hr(element)
            | Self::Html(element)
            | Self::I(element)
            | Self::Iframe(element)
            | Self::Image(element)
            | Self::Img(element)
            | Self::Input { element, .. }
            | Self::Ins(element)
            | Self::Isindex(element)
            | Self::Kbd(element)
            | Self::Keygen(element)
            | Self::Label(element)
            | Self::Legend(element)
            | Self::Li(element)
            | Self::Link(element)
            | Self::Main(element)
            | Self::Map(element)
            | Self::Mark(element)
            | Self::Marquee(element)
            | Self::Menu(element)
            | Self::Menuitem(element)
            | Self::Meta(element)
            | Self::Meter(element)
            | Self::Nav(element)
            | Self::Nobr(element)
            | Self::Noembed(element)
            | Self::Noframes(element)
            | Self::Noscript(element)
            | Self::Object(element)
            | Self::Ol(element)
            | Self::Optgroup(element)
            | Self::Option(element)
            | Self::Output(element)
            | Self::P(element)
            | Self::Param(element)
            | Self::Picture(element)
            | Self::Plaintext(element)
            | Self::Pre(element)
            | Self::Progress(element)
            | Self::Q(element)
            | Self::Rp(element)
            | Self::Rt(element)
            | Self::Ruby(element)
            | Self::S(element)
            | Self::Samp(element)
            | Self::Script(element)
            | Self::Section(element)
            | Self::Select(element)
            | Self::Small(element)
            | Self::Slot(element)
            | Self::Source(element)
            | Self::Spacer(element)
            | Self::Span(element)
            | Self::Strike(element)
            | Self::Strong(element)
            | Self::Style(element)
            | Self::Sub(element)
            | Self::Summary(element)
            | Self::Sup(element)
            | Self::Svg(element)
            | Self::Table(element)
            | Self::Tbody(element)
            | Self::Td(element)
            | Self::Template(element)
            | Self::Textarea(element)
            | Self::Tfoot(element)
            | Self::Th(element)
            | Self::Thead(element)
            | Self::Time(element)
            | Self::Title(element)
            | Self::Tr(element)
            | Self::Track(element)
            | Self::Tt(element)
            | Self::U(element)
            | Self::Ul(element)
            | Self::Var(element)
            | Self::Video(element)
            | Self::Wbr(element)
            | Self::Xmp(element)
            | Self::Unknown(element) => element,
        }
    }
    pub(crate) fn element_mut(&mut self) -> &mut Element {
        match self {
            Self::DocType(element)
            | Self::A(element)
            | Self::Abbr(element)
            | Self::Acronym(element)
            | Self::Address(element)
            | Self::Applet(element)
            | Self::Area(element)
            | Self::Article(element)
            | Self::Aside(element)
            | Self::Audio(element)
            | Self::B(element)
            | Self::Base(element)
            | Self::Basefont(element)
            | Self::Bdi(element)
            | Self::Bdo(element)
            | Self::Bgsound(element)
            | Self::Big(element)
            | Self::Blink(element)
            | Self::Blockquote(element)
            | Self::Body(element)
            | Self::Br(element)
            | Self::Button(element)
            | Self::Canvas(element)
            | Self::Caption(element)
            | Self::Center(element)
            | Self::Cite(element)
            | Self::Code(element)
            | Self::Col(element)
            | Self::Colgroup(element)
            | Self::Data(element)
            | Self::Datalist(element)
            | Self::Dd(element)
            | Self::Del(element)
            | Self::Details(element)
            | Self::Dfn(element)
            | Self::Dialog(element)
            | Self::Dir(element)
            | Self::Div(element)
            | Self::Dl(element)
            | Self::Dt(element)
            | Self::Em(element)
            | Self::Embed(element)
            | Self::Fieldset(element)
            | Self::Figcaption(element)
            | Self::Figure(element)
            | Self::Font(element)
            | Self::Footer(element)
            | Self::Form(element)
            | Self::Frame(element)
            | Self::Frameset(element)
            | Self::H1(element)
            | Self::H2(element)
            | Self::H3(element)
            | Self::H4(element)
            | Self::H5(element)
            | Self::H6(element)
            | Self::Head(element)
            | Self::Header(element)
            | Self::Hr(element)
            | Self::Html(element)
            | Self::I(element)
            | Self::Iframe(element)
            | Self::Image(element)
            | Self::Img(element)
            | Self::Input { element, .. }
            | Self::Ins(element)
            | Self::Isindex(element)
            | Self::Kbd(element)
            | Self::Keygen(element)
            | Self::Label(element)
            | Self::Legend(element)
            | Self::Li(element)
            | Self::Link(element)
            | Self::Main(element)
            | Self::Map(element)
            | Self::Mark(element)
            | Self::Marquee(element)
            | Self::Menu(element)
            | Self::Menuitem(element)
            | Self::Meta(element)
            | Self::Meter(element)
            | Self::Nav(element)
            | Self::Nobr(element)
            | Self::Noembed(element)
            | Self::Noframes(element)
            | Self::Noscript(element)
            | Self::Object(element)
            | Self::Ol(element)
            | Self::Optgroup(element)
            | Self::Option(element)
            | Self::Output(element)
            | Self::P(element)
            | Self::Param(element)
            | Self::Picture(element)
            | Self::Plaintext(element)
            | Self::Pre(element)
            | Self::Progress(element)
            | Self::Q(element)
            | Self::Rp(element)
            | Self::Rt(element)
            | Self::Ruby(element)
            | Self::S(element)
            | Self::Samp(element)
            | Self::Script(element)
            | Self::Section(element)
            | Self::Select(element)
            | Self::Small(element)
            | Self::Slot(element)
            | Self::Source(element)
            | Self::Spacer(element)
            | Self::Span(element)
            | Self::Strike(element)
            | Self::Strong(element)
            | Self::Style(element)
            | Self::Sub(element)
            | Self::Summary(element)
            | Self::Sup(element)
            | Self::Svg(element)
            | Self::Table(element)
            | Self::Tbody(element)
            | Self::Td(element)
            | Self::Template(element)
            | Self::Textarea(element)
            | Self::Tfoot(element)
            | Self::Th(element)
            | Self::Thead(element)
            | Self::Time(element)
            | Self::Title(element)
            | Self::Tr(element)
            | Self::Track(element)
            | Self::Tt(element)
            | Self::U(element)
            | Self::Ul(element)
            | Self::Var(element)
            | Self::Video(element)
            | Self::Wbr(element)
            | Self::Xmp(element)
            | Self::Unknown(element) => element,
        }
    }
}
