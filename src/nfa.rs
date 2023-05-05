use crate::PostfixExpr;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// A vertex in the NFA graph.
/// NFA中的一个节点
/// value: 该节点的值
/// neighbors: 该节点的邻居节点
pub struct StateVertex {
    pub neighbors: HashMap<u8, Rc<RefCell<StateVertex>>>,
    pub epsilon_neighbors: Vec<Rc<RefCell<StateVertex>>>,
}

impl StateVertex {
    pub fn new() -> StateVertex {
        StateVertex {
            neighbors: HashMap::new(),
            epsilon_neighbors: Vec::new(),
        }
    }
}

pub struct NFA {
    pub start: Rc<RefCell<StateVertex>>,
    pub end: Rc<RefCell<StateVertex>>,
}

pub fn to_nfa(expr: PostfixExpr) -> NFA {
    let mut expr = expr.0;
    let mut stack: Vec<NFA> = Vec::new();

    let mut left: Option<Rc<StateVertex>> = Option::None;
    let mut right: Option<Rc<StateVertex>> = Option::None;

    for each in expr.iter() {
        match each {
            b'|' => {
                // 对应或的逻辑
                // 按照固定的公式处理
                let curr_nfa = NFA {
                    start: Rc::new(RefCell::new(StateVertex::new())),
                    end: Rc::new(RefCell::new(StateVertex::new())),
                };

                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                // 添加epsilon-move

                left.end
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&curr_nfa.end));
                right
                    .end
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&curr_nfa.end));

                curr_nfa
                    .start
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&left.start));
                curr_nfa
                    .start
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&right.start));

                stack.push(curr_nfa);
            }
            b'*' => {
                // 对应闭包的逻辑
                // 按照固定的公式处理

                let left = stack.pop().unwrap();

                let curr_nfa = NFA {
                    start: Rc::new(RefCell::new(StateVertex::new())),
                    end: Rc::new(RefCell::new(StateVertex::new())),
                };

                // 添加epsilon-move
                curr_nfa
                    .start
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&left.start));
                curr_nfa
                    .start
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&curr_nfa.end));

                left.end
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&curr_nfa.start));
                left.end
                    .borrow_mut()
                    .epsilon_neighbors
                    .push(Rc::clone(&curr_nfa.end));

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
                    .push(Rc::clone(&right.start));

                stack.push(NFA {
                    start: Rc::clone(&left.start),
                    end: Rc::clone(&right.end),
                });
            }
            _ => {
                // 若只是字母，创建对应的节点，然后入栈
                // 例如：S0 --a--> S1
                let curr_nfa = NFA {
                    start: Rc::new(RefCell::new(StateVertex::new())),
                    end: Rc::new(RefCell::new(StateVertex::new())),
                };
                // 添加状态转移条件
                curr_nfa
                    .start
                    .borrow_mut()
                    .neighbors
                    .insert(*each, Rc::clone(&curr_nfa.end));
                stack.push(curr_nfa);
            }
        }
    }

    // 进行连接
    let mut right = stack.pop().unwrap();

    while let Some(left) = stack.pop() {
        // 添加epsilon-move
        left.end
            .borrow_mut()
            .epsilon_neighbors
            .push(Rc::clone(&right.start));

        right = NFA {
            start: Rc::clone(&left.start),
            end: Rc::clone(&right.end),
        };
    }

    while stack.len() > 1 {
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();

        // 添加epsilon-move
        left.end
            .borrow_mut()
            .epsilon_neighbors
            .push(Rc::clone(&right.start));

        stack.push(NFA {
            start: Rc::clone(&left.start),
            end: Rc::clone(&right.end),
        });
    }

    right
}
