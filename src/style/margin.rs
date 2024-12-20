use super::values::{r#box::BoxEdges, numeric::AutoOrLengthOrPercentage};

#[derive(Clone)]
pub struct Margin(BoxEdges<AutoOrLengthOrPercentage>);

impl Default for Margin {
    fn default() -> Self {
        let zero = AutoOrLengthOrPercentage::zero();
        Self(BoxEdges { top: zero, bottom: zero, left: zero, right: zero })
    }
}

pub mod initial {
    pub use super::Margin;
}

pub mod computed {
    pub use super::Margin;
}