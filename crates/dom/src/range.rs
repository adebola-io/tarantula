use crate::Node;

pub struct Range {
    pub(crate) is_live: bool,
    pub(crate) start: RangeMarker,
    pub(crate) end: RangeMarker,
}

pub(crate) struct RangeMarker {
    pub node: Node,
    pub offset: usize,
}
