use crate::Node;
use std::slice::Iter;

/// An iterator that traverses an operator tree in pre-order.
pub struct NodeIter<'a, IntType, FloatType> {
    stack: Vec<Iter<'a, Node<IntType, FloatType>>>,
}

impl<'a, IntType, FloatType> NodeIter<'a, IntType, FloatType> {
    fn new(node: &'a Node<IntType, FloatType>) -> Self {
        Self {
            stack: vec![node.children.iter()],
        }
    }
}

impl<'a, IntType, FloatType> Iterator for NodeIter<'a, IntType, FloatType> {
    type Item = &'a Node<IntType, FloatType>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut result = None;

            if let Some(last) = self.stack.last_mut() {
                if let Some(next) = last.next() {
                    result = Some(next);
                } else {
                    // Can not fail because we just borrowed last.
                    // We just checked that the iterator is empty, so we can safely discard it.
                    let _ = self.stack.pop().unwrap();
                }
            } else {
                return None;
            }

            if let Some(result) = result {
                self.stack.push(result.children.iter());
                return Some(result);
            }
        }
    }
}

impl<IntType, FloatType> Node<IntType, FloatType> {
    /// Returns an iterator over all nodes in this tree.
    pub fn iter(&self) -> impl Iterator<Item = &Node<IntType, FloatType>> {
        NodeIter::new(self)
    }
}
