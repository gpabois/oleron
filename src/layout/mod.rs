pub mod box_tree;
pub mod fragment_tree;

pub trait Lay<Layout> {
    fn lay(self, element: Self) -> Self;
}

pub struct Inline;
