use std::ops::{Deref, DerefMut};

use super::values::{numeric::AutoOrLengthOrPercentage, r#box::BoxEdges};

#[derive(Clone)]
pub struct Padding(BoxEdges<AutoOrLengthOrPercentage>);

impl Deref for Padding {
    type Target = BoxEdges<AutoOrLengthOrPercentage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Padding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Padding {
    fn default() -> Self {
        let zero = AutoOrLengthOrPercentage::zero();
        Self(BoxEdges {
            top: zero,
            bottom: zero,
            left: zero,
            right: zero,
        })
    }
}

pub mod initial {
    pub use super::Padding;
}

pub mod computed {
    pub use super::Padding;
}

