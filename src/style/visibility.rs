#[derive(Clone, Copy)]
pub enum Visibility {
    Collapse,
    Hidden,
    Visible,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Visible
    }
}

pub mod initial {
    pub use super::Visibility;
}

pub mod computed {
    pub use super::Visibility;
}

