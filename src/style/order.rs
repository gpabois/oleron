use super::values::numeric::Integer;

#[derive(Clone, Copy)]
pub struct Order(Integer);

impl Default for Order {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub mod initial {
    pub use super::Order;
}

pub mod computed {
    pub use super::Order;
}