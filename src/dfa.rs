use crate::{nfa::NfaVertexRef, Nfa};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

pub struct StateVertex {
    pub epsilon_closure: Vec<NfaVertexRef>,
    pub neighbors: HashMap<u8, DfaVertexRef>,
    pub handlers: HashSet<usize>,
}

pub struct DfaVertexRef(Rc<RefCell<StateVertex>>);

impl DfaVertexRef {
    pub fn new() -> Self {
        DfaVertexRef(Rc::new(RefCell::new(StateVertex {
            epsilon_closure: Vec::new(),
            neighbors: HashMap::new(),
            handlers: HashSet::new(),
        })))
    }
}

impl Deref for DfaVertexRef {
    type Target = Rc<RefCell<StateVertex>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for DfaVertexRef {
    fn clone(&self) -> Self {
        DfaVertexRef(Rc::clone(&self.0))
    }
}

impl PartialEq for DfaVertexRef {
    fn eq(&self, other: &Self) -> bool {
        let closure1 = &self.borrow().epsilon_closure;
        let closure2 = &other.borrow().epsilon_closure;
        if closure1.len() != closure2.len() {
            return false;
        }

        closure1.iter().all(|each| closure2.contains(each))
    }
}

impl NfaVertexRef {
    /// epsilon_closure
    /// get epsilon closure of a vertex
    /// result will be stored in visited
    fn epsilon_closure(&self, visited: &mut Vec<NfaVertexRef>) {
        if visited.contains(self) {
            return;
        }

        // 将自身添加到闭包中
        visited.push(NfaVertexRef::clone(self));

        // 遍历相邻节点
        self.borrow().epsilon_neighbors.iter().for_each(|each| {
            each.epsilon_closure(visited);
        });
    }
}

#[derive(Serialize, Deserialize)]
pub struct LookupState {
    pub handlers: HashSet<usize>,
    pub neighbors: HashMap<u8, usize>,
}

impl LookupState {
    pub fn new() -> Self {
        LookupState {
            handlers: HashSet::new(),
            neighbors: HashMap::new(),
        }
    }

    pub fn insert_neighbor(&mut self, ch: u8, index: usize) {
        self.neighbors.insert(ch, index);
    }
}

#[derive(Serialize, Deserialize)]
pub struct LookupTable {
    pub states: Vec<LookupState>,
}

impl LookupTable {
    pub fn new() -> Self {
        LookupTable { states: Vec::new() }
    }
}

pub struct Dfa {
    pub vertexs: Vec<DfaVertexRef>,
    pub lookup_table: LookupTable,
}

impl Dfa {
    /// build
    /// build dfa from nfa
    /// return start vertex of dfa
    pub fn build(nfa: &Nfa) -> Dfa {
        // 以nfa的开始节点的epsilon-closure为开始节点
        let start = DfaVertexRef::new();

        nfa.start
            .epsilon_closure(&mut start.borrow_mut().epsilon_closure);

        // 递归扩展
        let mut visited = Vec::new();
        Self::tarverse_vertex(DfaVertexRef::clone(&start), &mut visited);

        // 标记每个节点的handler
        visited.iter().for_each(|each| {
            let mut handlers = HashSet::new();
            each.borrow()
                .epsilon_closure
                .iter()
                .for_each(|each_closure| {
                    if let Some(handler) = each_closure.borrow().handler {
                        handlers.insert(handler);
                    }
                });

            each.borrow_mut().handlers = handlers;
        });

        let mut lookup_table = LookupTable::new();
        // 生成lookup table
        visited.iter().enumerate().for_each(|(_, each)| {
            let mut curr_state = LookupState::new();

            // 指定handler
            curr_state.handlers = each.borrow().handlers.clone();

            // 遍历所有的邻居，写到转换表中
            each.borrow()
                .neighbors
                .iter()
                .for_each(|(&cond, neighbor)| {
                    let id = visited
                        .iter()
                        .position(|each| (*each) == (*neighbor))
                        .unwrap();
                    curr_state.insert_neighbor(cond, id);
                });

            lookup_table.states.push(curr_state);
        });

        Dfa {
            vertexs: visited,
            lookup_table,
        }
    }

    pub fn get_start(&self) -> DfaVertexRef {
        DfaVertexRef::clone(&self.vertexs[0])
    }

    // 扩展当前dfa节点
    fn tarverse_vertex(vertex: DfaVertexRef, visited: &mut Vec<DfaVertexRef>) {
        // 如果当前节点已经访问过，则返回
        // 否则，将自身添加到已访问节点中
        if visited.contains(&vertex) {
            return;
        }
        visited.push(DfaVertexRef::clone(&vertex));

        // 遍历当前节点的epsilon-closure
        // 求出closure中每个元素的可以到达的节点的epsilon-closure
        // 并且将其添加到当前节点的neighbors中
        let mut neighbors: HashMap<u8, DfaVertexRef> = HashMap::new();

        vertex
            .borrow()
            .epsilon_closure
            .iter()
            .for_each(|each_closure| {
                // 遍历每个闭包的相邻节点
                each_closure
                    .borrow()
                    .neighbors
                    .iter()
                    .for_each(|(&cond, nfa_vertex)| {
                        // 如果对应当前状态的转换条件不存在，则创建
                        let cond_neighbor = match neighbors.get(&cond) {
                            Some(each) => DfaVertexRef::clone(each),
                            None => {
                                let new_neighbor = DfaVertexRef::new();
                                neighbors.insert(cond, DfaVertexRef::clone(&new_neighbor));
                                new_neighbor
                            }
                        };

                        // 计算改nfa节点的epsilon-closure
                        let mut closure = Vec::new();
                        nfa_vertex.epsilon_closure(&mut closure);

                        // 遍历该闭包中的每个节点
                        // 如果当前转换条件的邻居节点的epsilon-closure中已经包含了该节点，则不添加
                        // 否则，添加到该节点的epsilon-closure中
                        closure.iter().for_each(|each| {
                            if cond_neighbor.borrow().epsilon_closure.contains(each) {
                                return;
                            }

                            cond_neighbor
                                .borrow_mut()
                                .epsilon_closure
                                .push(NfaVertexRef::clone(each));
                        })
                    });
            });

        // 遍历所有的neightbors
        // 如果visited中已经出现过该节点，则用visited中的节点替换
        neighbors.iter_mut().for_each(|(_, vertex)| {
            if let Some(same) = visited.iter().find(|&each_visited| each_visited == vertex) {
                *vertex = DfaVertexRef::clone(same);
            }
        });

        // 将neighbors添加到当前节点的neighbors中
        vertex.borrow_mut().neighbors = neighbors;

        // 递归扩展
        vertex.borrow().neighbors.iter().for_each(|(_, neighbor)| {
            Self::tarverse_vertex(DfaVertexRef::clone(neighbor), visited)
        });
    }
}
