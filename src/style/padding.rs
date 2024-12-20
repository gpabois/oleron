use super::values::{r#box::BoxEdges, numeric::AutoOrLengthOrPercentage};

#[derive(Clone)]
pub struct Padding(BoxEdges<AutoOrLengthOrPercentage>);

impl Default for Padding {
    fn default() -> Self {
        let zero = AutoOrLengthOrPercentage::zero();
        Self(BoxEdges { top: zero, bottom: zero, left: zero, right: zero })
    }
}

pub mod initial {
    pub use super::Padding;
}

pub mod computed {
    pub use super::Padding;
}