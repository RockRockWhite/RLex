use crate::RegexExpr;
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

/// A vertex in the NFA graph.
/// NFA中的一个节点
/// value: 该节点的值
/// neighbors: 该节点的邻居节点
pub struct StateVertex {
    pub neighbors: HashMap<u8, NfaVertexRef>,
    pub epsilon_neighbors: Vec<NfaVertexRef>,
}

impl StateVertex {
    pub fn new() -> StateVertex {
        StateVertex {
            neighbors: HashMap::new(),
            epsilon_neighbors: Vec::new(),
        }
    }
}

pub struct NfaVertexRef(Rc<RefCell<StateVertex>>);

impl NfaVertexRef {
    pub fn new() -> NfaVertexRef {
        NfaVertexRef(Rc::new(RefCell::new(StateVertex::new())))
    }
}

impl Clone for NfaVertexRef {
    fn clone(&self) -> Self {
        NfaVertexRef(Rc::clone(&self.0))
    }
}

impl Deref for NfaVertexRef {
    type Target = Rc<RefCell<StateVertex>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for NfaVertexRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub struct Nfa {
    pub start: NfaVertexRef,
    pub end: NfaVertexRef,
}

impl Nfa {
    pub fn build(expr: &RegexExpr) -> Nfa {
        let expr = &expr.0;
        let mut stack: Vec<Nfa> = Vec::new();

        // let mut left: Option<Rc<StateVertex>> = Option::None;
        // let mut right: Option<Rc<StateVertex>> = Option::None;

        for each in expr.iter() {
            match each {
                b'|' => {
                    // 对应或的逻辑
                    // 按照固定的公式处理
                    let curr_nfa = Nfa {
                        start: NfaVertexRef::new(),
                        end: NfaVertexRef::new(),
                    };

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // 添加epsilon-move

                    left.end
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&curr_nfa.end));
                    right
                        .end
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&curr_nfa.end));

                    curr_nfa
                        .start
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&left.start));
                    curr_nfa
                        .start
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&right.start));

                    stack.push(curr_nfa);
                }
                b'*' => {
                    // 对应闭包的逻辑
                    // 按照固定的公式处理

                    let left = stack.pop().unwrap();

                    let curr_nfa = Nfa {
                        start: NfaVertexRef::new(),
                        end: NfaVertexRef::new(),
                    };

                    // 添加epsilon-move
                    curr_nfa
                        .start
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&left.start));
                    curr_nfa
                        .start
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&curr_nfa.end));

                    left.end
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&curr_nfa.start));
                    left.end
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&curr_nfa.end));

                    stack.push(curr_nfa);
                }
                b'.' => {
                    // 对应连接的逻辑
                    // 按照固定的公式处理
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // 添加epsilon-move
                    left.end
                        .borrow_mut()
                        .epsilon_neighbors
                        .push(NfaVertexRef::clone(&right.start));

                    stack.push(Nfa {
                        start: NfaVertexRef::clone(&left.start),
                        end: NfaVertexRef::clone(&right.end),
                    });
                }
                _ => {
                    // 若只是字母，创建对应的节点，然后入栈
                    // 例如：S0 --a--> S1
                    let curr_nfa = Nfa {
                        start: NfaVertexRef::new(),
                        end: NfaVertexRef::new(),
                    };
                    // 添加状态转移条件
                    curr_nfa
                        .start
                        .borrow_mut()
                        .neighbors
                        .insert(*each, NfaVertexRef::clone(&curr_nfa.end));
                    stack.push(curr_nfa);
                }
            }
        }

        stack.pop().unwrap()
    }
}
