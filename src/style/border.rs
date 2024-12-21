use std::ops::{Deref, DerefMut};

use super::values::{numeric::AutoOrLengthOrPercentage, r#box::BoxEdges};

#[derive(Clone)]
pub struct Border(BoxEdges<AutoOrLengthOrPercentage>);

impl Deref for Border {
    type Target = BoxEdges<AutoOrLengthOrPercentage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Border {
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
    pub use super::Border;
}

pub mod computed {
    pub use super::Border;
}

