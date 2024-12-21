use std::collections::VecDeque;

use crate::{
    ecs::systems::{boxes::BoxSystem, tree::TreeSystem},
    r#box::Box,
};

use super::{
    fragment_tree::{Fragment, FragmentKind, FragmentTree},
    Block, Inline, Lay, Layout,
};

pub trait IsFragmentable<FormattingContext> {
    fn is_fragmentable(&self, fragment: &Fragment) -> bool;
}

/// Perform fragmentation each time a break is encountered.
pub fn fragment(fragment_tree: &mut FragmentTree, fragment: &Fragment) {
    if !fragment_tree.is_breakable(fragment) {
        return;
    }

    let children = fragment_tree
        .iter_children(fragment)
        .map(|child| (child, fragment_tree.is_break(fragment)))
        .collect::<Vec<_>>();
    let mut previous = *fragment;
    let clusters = children
        .split(|(_, is_break)| *is_break)
        .collect::<Vec<_>>();

    if clusters.len() <= 1 {
        return;
    }

    for cluster in clusters {
        let brk = fragment_tree.insert_break();
        let clone = fragment_tree.clone_fragment(&fragment);

        fragment_tree.attach_children(&clone, cluster.iter().map(|(child, _)| child).copied());

        fragment_tree.push_sibling(&previous, brk);
        fragment_tree.push_sibling(&brk, clone);

        previous = clone;
    }
}

/// Breaking introduces break fragment
/// which is used as an indicator to fragment a parent node.
pub trait Break<FormattingContext> {
    fn r#break(&self, fragment_tree: &mut FragmentTree, fragment: &Fragment) -> bool;
}

/// Break if the node's width is gt max width.
///
/// It is an *unforced* break
pub struct OverflowBreak {
    pub max_length: i32,
}

impl Break<Inline> for OverflowBreak {
    fn r#break(&self, fragment_tree: &mut FragmentTree, fragment: &Fragment) -> bool {
        let r#box = fragment_tree.borrow_box(&fragment).unwrap();

        // We don't need to fragment it.
        if r#box.content.width <= self.max_length {
            return false;
        }

        drop(r#box);

        // We cannot fragment it.
        if IsFragmentable::<Inline>::is_fragmentable(fragment_tree, fragment) {
            return false;
        }

        let mut acc = Box::<i32>::default();

        // We find a possible break point
        let child_boxes = fragment_tree
            .iter_children(fragment)
            .map(|child| {
                (
                    child,
                    fragment_tree.is_break(&child),
                    fragment_tree.borrow_box(&child).map(|c| c.clone()),
                )
            })
            .collect::<Vec<_>>();

        for (child, is_break, maybe_box) in child_boxes {
            if is_break {
                return true;
            }

            if let Some(r#box) = maybe_box {
                acc = Lay::<Inline>::lay(acc, r#box);
            }

            if acc.content.width > self.max_length {
                // We can fragment it
                if IsFragmentable::<Inline>::is_fragmentable(fragment_tree, fragment) {
                    let sub_break = Self {
                        max_length: acc.content.width - self.max_length,
                    };

                    return sub_break.r#break(fragment_tree, &child);
                } else {
                    // we need to break before or after the monolith.
                    // We have fragments before we break it before
                    let maybe_at = if let Some(previous) = fragment_tree.before(&child) {
                        Some(previous)
                    } else if let Some(next) = fragment_tree.after(&child) {
                        // We break afterwards
                        Some(next)
                    } else {
                        None
                    };

                    if let Some(at) = maybe_at {
                        let brk = fragment_tree.insert_break();
                        fragment_tree.push_sibling(&at, brk);
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

pub struct LineBreak;

impl<U> Break<U> for LineBreak {
    /// Fragment a text sequence on a line break.
    ///
    /// # Arguments
    /// - `text_sequence` - Must be a text sequence fragment
    fn r#break(&self, fragment_tree: &mut FragmentTree, node: &Fragment) -> bool {
        if let FragmentKind::TextSequence = fragment_tree.kind(node) {
            let mut seq = fragment_tree.text_sequences.borrow_mut(node).unwrap();
            let mut frag_seq = seq.split_by_line_breaks().collect::<VecDeque<_>>();

            if let Some(head) = frag_seq.pop_front() {
                *seq = head;
            }

            drop(seq);

            let mut previous = *node;
            let has_break = frag_seq.len() > 0;
            while let Some(sub_seq) = frag_seq.pop_front() {
                let brk = fragment_tree.insert_break();
                fragment_tree.push_sibling(&previous, brk);

                let seq_frag = fragment_tree.insert_text_sequence(sub_seq);
                fragment_tree.push_sibling(&brk, seq_frag);
                previous = seq_frag;
            }

            return has_break;
        }

        return false;
    }
}
