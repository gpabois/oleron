use std::ops::Deref;

use super::values::numeric::Integer;

#[derive(Clone, Copy, Default)]
pub struct Order(Integer);

impl Deref for Order {
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod initial {
    pub use super::Order;
}

pub mod computed {
    pub use super::Order;
}

