use pb_arena::{Arena, ArenaId};

use crate::{component::Components, font::SizedFont};

pub type Fragment = ArenaId;

pub struct FragmentTree {
    nodes: Arena<FragmentKind>,
    edges: Components<Fragment, FragmentEdge>,
    boxes: Components<Fragment, Box<i32>>,
    glyph_clusters: Components<Fragment, GlyphCluster>
}

impl FragmentTree {
    /// Push a sibling in the linked list.
    pub fn push_sibling(&self, node: Fragment, new_sibling: Fragment) {
        let mut maybe_old_sibling: Option<Fragment> = None;

        if let Some(mut edges) = self.edges.borrow_mut(&node) {
            maybe_old_sibling = edges.sibling;
            edges.sibling = Some(new_sibling);
        }

        if let Some(old_sibling) = maybe_old_sibling {
            self.push_sibling(new_sibling, old_sibling);
        }
    }
}

pub struct FragmentEdge {
    sibling: Option<Fragment>,
    parent: Option<Fragment>
}

pub enum FragmentKind {
    GlyphMatrix,
    GlyphCluster,
    LineBreak,
    LineBox
}

pub struct GlyphCluster {
    text: String,
    font: SizedFont
}



