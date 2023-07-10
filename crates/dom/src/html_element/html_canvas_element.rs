use std::any::Any;

use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml, MediaStream,
};

pub struct CanvasRenderingContext2DSettings;
pub struct ImageBitmapRenderingContextSettings;
pub struct ImageBitmapRenderingContext;
pub struct CanvasRenderingContext2D;
pub struct WebGLContextAttributes;
pub struct WebGLRenderingContext;
pub struct RenderingContext;

pub struct WebGL2RenderingContext;
pub struct BlobCallBack;
pub struct OffscreenCanvas;

pub struct HTMLCanvasElement {
    value: HTMLElement,
}

// Properties.
impl HTMLCanvasElement {
    pub fn height(&self) -> usize {
        todo!()
    }
    pub fn set_height(&mut self, value: usize) {
        todo!()
    }
    pub fn width(&self) -> usize {
        todo!()
    }
    pub fn set_width(&mut self, value: usize) {
        todo!()
    }
}

// Methods.
impl HTMLCanvasElement {
    pub fn capture_stream(&mut self, frame_request_rate: usize) -> MediaStream {
        todo!()
    }
    pub fn get_context(&self, context_id: &str, options: impl Any) -> Option<RenderingContext> {
        todo!()
    }
    pub fn get_2d_context(
        &self,
        options: Option<CanvasRenderingContext2DSettings>,
    ) -> Option<CanvasRenderingContext2D> {
        todo!()
    }
    pub fn get_bitmaprenderer_context(
        &self,
        options: Option<ImageBitmapRenderingContextSettings>,
    ) -> Option<ImageBitmapRenderingContext> {
        todo!()
    }
    pub fn get_webgl(
        &self,
        options: Option<WebGLContextAttributes>,
    ) -> Option<WebGLRenderingContext> {
        todo!()
    }
    pub fn get_webgl2_context(
        &self,
        options: Option<WebGLContextAttributes>,
    ) -> Option<WebGL2RenderingContext> {
        todo!()
    }
    pub fn to_blob(
        &mut self,
        callback: BlobCallBack,
        r#type: Option<&str>,
        quality: Option<impl Any>,
    ) {
        todo!()
    }
    pub fn to_data_url(&self, r#type: Option<&str>, quality: Option<impl Any>) -> String {
        todo!()
    }
    pub fn transfer_control_to_offscreen(&self) -> OffscreenCanvas {
        todo!()
    }
}

impl AsHTMLElement for HTMLCanvasElement {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for HTMLCanvasElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLCanvasElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLCanvasElement {}
impl AsChildNode for HTMLCanvasElement {}
impl AsNode for HTMLCanvasElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.value)
    }

    fn node_name(&self) -> String {
        self.value.tag_name()
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLCanvasElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLCanvasElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLCanvasElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLCanvasElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::A) {
            Ok(HTMLCanvasElement { value })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLCanvasElement"
            )))
        }
    }
}
