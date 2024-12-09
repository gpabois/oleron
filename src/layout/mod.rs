pub mod box_tree;
pub mod fragment_tree;

pub trait LayIn<Layout> {
    fn lay_in(&mut self, element: Self);
}

pub struct Inline;
