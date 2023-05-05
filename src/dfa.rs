use std::{cell::RefCell, rc::Rc};

use crate::nfa::StateVertex as NFAStateVertex;

/// epsilon_closure
/// get epsilon closure of a vertex
/// result will be stored in visited
pub fn epsilon_closure(
    vertex: Rc<RefCell<NFAStateVertex>>,
    visited: &mut Vec<Rc<RefCell<NFAStateVertex>>>,
) {
    visited.iter().any(|each| {
        if Rc::ptr_eq(&vertex, each) {
            return true;
        }
        false
    });

    // 将自身添加到闭包中
    visited.push(Rc::clone(&vertex));

    // 遍历相邻节点
    vertex.borrow().epsilon_neighbors.iter().for_each(|each| {
        epsilon_closure(Rc::clone(&each), visited);
    });
}
