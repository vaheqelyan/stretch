//! Forest - ECS like datastructure for storing node trees.
//!
//! Backing datastructure for `Stretch` structs.

use crate::geometry::Size;
use crate::id::NodeId;
use crate::node::MeasureFunc;
use crate::number::Number;
use crate::result::{Cache, Layout};
use crate::style::{Style, Dimension};
use crate::Error;

pub(crate) struct NodeData {
    pub(crate) style: Style,
    pub(crate) measure: Option<MeasureFunc>,
    pub(crate) layout: Layout,
    pub(crate) layout_cache: Option<Cache>,
    pub(crate) is_dirty: bool,
    pub(crate) scroll_view: bool,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) bottom: f32,
    pub(crate) right: f32,
    pub(crate) offset: f32,
    pub(crate) cache_el_count: u32,
    pub(crate) cache_farest_element: f32,
}

impl NodeData {
    fn new_leaf(style: Style, measure: MeasureFunc) -> Self {
        NodeData { cache_el_count: 0, cache_farest_element: 0.0, offset: 0.0, bottom:0.0, right:0.0, x: 0.0, y:0.0, scroll_view: false,style, measure: Some(measure), layout_cache: None, layout: Layout::new(), is_dirty: true, }
    }
    
    fn new_scroll_view(style: Style) -> Self {
        NodeData { cache_el_count: 0, cache_farest_element: 0.0, offset: 0.0,bottom:0.0, right:0.0, x: 0.0, y:0.0, style, measure: None, layout_cache: None, layout: Layout::new(), is_dirty: true, scroll_view: true }
    }

    fn new(style: Style) -> Self {
        NodeData { cache_el_count: 0, cache_farest_element: 0.0, offset: 0.0,bottom:0.0, right:0.0, x:0.0, y:0.0, scroll_view: false, style, measure: None, layout_cache: None, layout: Layout::new(), is_dirty: true, }
    }
}

pub(crate) struct Forest {
    pub(crate) nodes: Vec<NodeData>,
    pub(crate) children: Vec<Vec<NodeId>>,
    pub(crate) parents: Vec<Vec<NodeId>>,
}

impl Forest {
    pub fn with_capacity(capacity: usize) -> Self {
        Forest {
            nodes: Vec::with_capacity(capacity),
            children: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(NodeData::new_leaf(style, measure));
        self.children.push(Vec::with_capacity(0));
        self.parents.push(Vec::with_capacity(1));
        id
    }
   

    pub fn new_node(&mut self, style: Style, children: Vec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new(style));
        self.children.push(children);
        self.parents.push(Vec::with_capacity(1));
        id
    }
    
    
    pub fn new_scroll_view(&mut self, style: Style, children: Vec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        for child in &children {
            self.parents[*child].push(id);
        }
        self.nodes.push(NodeData::new_scroll_view(style));
        self.children.push(children);
        self.parents.push(Vec::with_capacity(1));
        id
    }


    pub fn add_child(&mut self, node: NodeId, child: NodeId) {
        self.parents[child].push(node);
        self.children[node].push(child);
        self.mark_dirty(node)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Removes a node and swaps with the last node.
    pub fn swap_remove(&mut self, node: NodeId) -> Option<NodeId> {
        self.nodes.swap_remove(node);

        // Now the last element is swapped in at index `node`.
        if self.nodes.is_empty() {
            self.children.clear();
            self.parents.clear();
            return None;
        }

        // Remove old node as parent from all its chilren.
        for child in &self.children[node] {
            let parents_child = &mut self.parents[*child];
            let mut pos = 0;
            while pos < parents_child.len() {
                if parents_child[pos] == node {
                    parents_child.swap_remove(pos);
                } else {
                    pos += 1;
                }
            }
        }

        // Remove old node as child from all its parents.
        for parent in &self.parents[node] {
            let childrens_parent = &mut self.children[*parent];
            let mut pos = 0;
            while pos < childrens_parent.len() {
                if childrens_parent[pos] == node {
                    childrens_parent.swap_remove(pos);
                } else {
                    pos += 1;
                }
            }
        }

        let last = self.nodes.len();

        return if last != node {
            // Update ids for every child of the swapped in node.
            for child in &self.children[last] {
                for parent in &mut self.parents[*child] {
                    if *parent == last {
                        *parent = node;
                    }
                }
            }

            // Update ids for every parent of the swapped in node.
            for parent in &self.parents[last] {
                for child in &mut self.children[*parent] {
                    if *child == last {
                        *child = node;
                    }
                }
            }

            self.children.swap_remove(node);
            self.parents.swap_remove(node);

            Some(last)
        } else {
            self.children.swap_remove(node);
            self.parents.swap_remove(node);
            None
        };
    }

    pub unsafe fn remove_child(&mut self, node: NodeId, child: NodeId) -> NodeId {
        let index = self.children[node].iter().position(|n| *n == child).unwrap();
        self.remove_child_at_index(node, index)
    }

    pub fn remove_child_at_index(&mut self, node: NodeId, index: usize) -> NodeId {
        let child = self.children[node].remove(index);
        self.parents[child].retain(|p| *p != node);
        self.mark_dirty(node);
        child
    }
    
    pub fn set_offset(&mut self, node: NodeId, offset: f32) {
        fn set_offset(nodes: &mut Vec<NodeData>, node_id: NodeId, offset: f32) {
            let node = &mut nodes[node_id];
            node.offset += offset;
            if node.offset <= 0.0 {
                node.offset = 0.0;
            }
        }
        set_offset(&mut self.nodes, node, offset);
    }
    
    
    pub fn set_pos(&mut self, node: NodeId, x: f32, y: f32, bottom: f32, right: f32) {
        fn set_pos(nodes: &mut Vec<NodeData>, node_id: NodeId, x: f32, y: f32, bottom: f32, right: f32) {
            let node = &mut nodes[node_id];
            node.x = x;
            node.y = y;
            node.bottom = bottom;
            node.right = right;
        }
        set_pos(&mut self.nodes, node, x, y, bottom, right);
    }
    
    pub fn set_cache(&mut self, node: NodeId, el_count: u32, far_el: f32) {
        fn set_cache(nodes: &mut Vec<NodeData>, node_id: NodeId, el_count: u32, far_el: f32) {
            let node = &mut nodes[node_id];
            node.cache_el_count = el_count;
            node.cache_farest_element = far_el;
        }
        set_pos(&mut self.nodes, node, el_count, far_el);
    }

    pub fn mark_dirty(&mut self, node: NodeId) {
        fn mark_dirty_impl(nodes: &mut Vec<NodeData>, parents: &Vec<Vec<NodeId>>, node_id: NodeId) {
            let node = &mut nodes[node_id];
            node.layout_cache = None;
            node.is_dirty = true;

            for parent in &parents[node_id] {
                mark_dirty_impl(nodes, parents, *parent);
            }
        }

        mark_dirty_impl(&mut self.nodes, &self.parents, node);
    }

    pub fn compute_layout(&mut self, node: NodeId, size: Size<Number>) -> Result<(), Error> {
        self.compute(node, size).map_err(|err| Error::Measure(err))
    }
}
