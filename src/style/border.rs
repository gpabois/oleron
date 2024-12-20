use super::values::{r#box::BoxEdges, numeric::AutoOrLengthOrPercentage};

#[derive(Clone)]
pub struct Border(BoxEdges<AutoOrLengthOrPercentage>);

impl Default for Border {
    fn default() -> Self {
        let zero = AutoOrLengthOrPercentage::zero();
        Self(BoxEdges { top: zero, bottom: zero, left: zero, right: zero })
    }
}

pub mod initial {
    pub use super::Border;
}

pub mod computed {
    pub use super::Border;
}