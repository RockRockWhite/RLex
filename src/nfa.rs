use crate::{regex_expr::Charactor, RegexExpr};
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};
/// A vertex in the NFA graph.
/// NFA中的一个节点
/// value: 该节点的值
/// neighbors: 该节点的邻居节点
pub struct StateVertex {
    pub neighbors: HashMap<u8, NfaVertexRef>,
    pub epsilon_neighbors: Vec<NfaVertexRef>,
    pub handler: Option<usize>,
}

impl StateVertex {
    pub fn new() -> StateVertex {
        StateVertex {
            neighbors: HashMap::new(),
            epsilon_neighbors: Vec::new(),
            handler: Option::None,
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
    pub fn build(expr: &RegexExpr, handler_id: usize) -> Nfa {
        let expr = &expr.0;
        let mut stack: Vec<Nfa> = Vec::new();

        // let mut left: Option<Rc<StateVertex>> = Option::None;
        // let mut right: Option<Rc<StateVertex>> = Option::None;

        for each in expr.iter() {
            match each {
                Charactor::Or => {
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
                Charactor::Closure => {
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
                Charactor::Concat => {
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
                Charactor::Char(c) => {
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
                        .insert(*c, NfaVertexRef::clone(&curr_nfa.end));
                    stack.push(curr_nfa);
                }
                _ => {}
            }
        }

        let res = stack.pop().unwrap();
        // 标记终止状态handler
        res.end.borrow_mut().handler = Some(handler_id);
        res
    }
}

pub struct NfaBuilder {
    pub nfa: Option<Nfa>,
}

impl NfaBuilder {
    pub fn new() -> NfaBuilder {
        NfaBuilder { nfa: Option::None }
    }

    pub fn add_rule(&mut self, expr: &RegexExpr, handler_id: usize) {
        let res;

        if let Some(left) = self.nfa.take() {
            let right = Nfa::build(expr, handler_id);

            // 为两个NFA添加Or逻辑
            res = Nfa {
                start: NfaVertexRef::new(),
                end: NfaVertexRef::new(),
            };
            // 添加epsilon-move
            left.end
                .borrow_mut()
                .epsilon_neighbors
                .push(NfaVertexRef::clone(&res.end));
            right
                .end
                .borrow_mut()
                .epsilon_neighbors
                .push(NfaVertexRef::clone(&res.end));

            res.start
                .borrow_mut()
                .epsilon_neighbors
                .push(NfaVertexRef::clone(&left.start));
            res.start
                .borrow_mut()
                .epsilon_neighbors
                .push(NfaVertexRef::clone(&right.start));
        } else {
            res = Nfa::build(expr, handler_id);
        }

        self.nfa = Option::Some(res);
    }

    pub fn build(&mut self) -> Option<Nfa> {
        self.nfa.take()
    }
}
