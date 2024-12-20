use std::hash::Hash;

use crate::{dom::TDocumentObjectModelExplorer, ecs::systems::tree::walk, style::Style};


pub struct Layout;

pub fn layout<DOM>(dom: &DOM, style: Style<DOM::NodeId>) 
where DOM: TDocumentObjectModelExplorer + Sync, DOM::NodeId: Hash + Copy + Eq
{
    for node in walk(dom) {
        
    }
}