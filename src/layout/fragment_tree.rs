use crate::font::SizedFont;

pub struct FragmentTree {
    
}

pub enum FragmentKind {
    GlyphMatrix,
    GlyphCluster,
}

pub struct GlyphMatrix {
    ch: char
}

pub struct GlyphCluster {
    text: String,
    font: SizedFont
}

