use std::ops::{Deref, DerefMut};

use super::values::{numeric::AutoOrLengthOrPercentage, r#box::BoxEdges};

#[derive(Clone)]
pub struct Margin(BoxEdges<AutoOrLengthOrPercentage>);

impl Deref for Margin {
    type Target = BoxEdges<AutoOrLengthOrPercentage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Margin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Margin {
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
    pub use super::Margin;
}

pub mod computed {
    pub use super::Margin;
}

