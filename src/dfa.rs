use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::{nfa::StateVertex as NFAStateVertex, NFA};

pub struct DFAStateVertex {
    pub acceptable: bool,
    pub epsilon_closure: Vec<Rc<RefCell<NFAStateVertex>>>,
    pub neighbors: HashMap<u8, Rc<RefCell<DFAStateVertex>>>,
}

pub struct DFA(Rc<RefCell<DFAStateVertex>>);

impl DFA {
    pub fn new() -> Self {
        DFA(Rc::new(RefCell::new(DFAStateVertex {
            acceptable: false,
            epsilon_closure: Vec::new(),
            neighbors: HashMap::new(),
        })))
    }
}

/// epsilon_closure
/// get epsilon closure of a vertex
/// result will be stored in visited
fn epsilon_closure(
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

/// to_dfa
/// convert nfa to dfa
/// return start vertex of dfa
pub fn to_dfa(nfa: &NFA) -> Rc<RefCell<DFAStateVertex>> {
    // 以nfa的开始节点的epsilon-closure为开始节点
    let start = Rc::new(RefCell::new(DFAStateVertex::new()));
    epsilon_closure(
        Rc::clone(&nfa.start),
        &mut start.borrow_mut().epsilon_closure,
    );

    // 递归扩展
    let mut visited = Vec::new();
    tarverse_vertex(Rc::clone(&start), &mut visited);

    println!("visited: {}", visited.len());

    start
}

// 扩展当前dfa节点
pub fn tarverse_vertex(
    vertex: Rc<RefCell<DFAStateVertex>>,
    visited: &mut Vec<Rc<RefCell<DFAStateVertex>>>,
) {
    // 如果当前节点已经访问过，则返回
    // 否则，将自身添加到已访问节点中
    if visited.iter().any(|each| {
        if Rc::ptr_eq(&vertex, each) {
            return true;
        }
        false
    }) {
        return;
    }
    visited.push(Rc::clone(&vertex));

    let mut neighbors: HashMap<u8, Rc<RefCell<DFAStateVertex>>> = HashMap::new();
    vertex
        .borrow()
        .epsilon_closure
        .iter()
        .for_each(|each_nfa_vertex| {
            // 遍历闭包中的每个节点的相邻节点
            each_nfa_vertex
                .borrow()
                .neighbors
                .iter()
                .for_each(|(key, value)| {
                    // 如果对应的转换状态不存在，则创建
                    if !neighbors.contains_key(key) {}

                    let neighbor = match neighbors.get(key) {
                        Some(each) => Rc::clone(each),
                        None => {
                            let new_neighbor = Rc::new(RefCell::new(DFAStateVertex::new()));
                            neighbors.insert(*key, Rc::clone(&new_neighbor));
                            new_neighbor
                        }
                    };

                    // 求出对应的epsilon-closure
                    let mut closure = Vec::new();
                    epsilon_closure(Rc::clone(&value), &mut neighbor.borrow_mut().closure);

                    if neighbor.borrow().epsilon_closure.iter().any(|each| {
                        if Rc::ptr_eq(&value, each) {
                            return true;
                        }
                        false
                    }) {
                        return;
                    }

                    neighbor
                        .borrow_mut()
                        .epsilon_closure
                        .push(Rc::clone(&value));
                });
        });

    // 遍历当前节点的相邻节点
    // 如果有和访问过的重复，则替换为已访问的节点
    // 否则，递归扩展
    neighbors.iter_mut().for_each(|(_, value)| {
        let mut matched_visited: Option<Rc<RefCell<DFAStateVertex>>> = None;
        visited.iter().for_each(|each_visited| {
            if each_visited.borrow().epsilon_closure.len() != value.borrow().epsilon_closure.len() {
                return;
            }

            // 判断closure是否相等
            if each_visited
                .borrow()
                .epsilon_closure
                .iter()
                .all(|each_closure| {
                    value.borrow().epsilon_closure.iter().any(|each| {
                        if Rc::ptr_eq(&each_closure, each) {
                            return true;
                        }
                        false
                    })
                })
            {
                matched_visited = Some(Rc::clone(each_visited));
            }
        });

        // 替换
        if let Some(matched_visited) = matched_visited.take() {
            *value = matched_visited;
        }
    });

    // 更新neighbors
    vertex.borrow_mut().neighbors = neighbors;
    // 递归扩展
    vertex.borrow().neighbors.iter().for_each(|(_, value)| {
        tarverse_vertex(Rc::clone(value), visited);
    });
}
