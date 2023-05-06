use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use crate::{nfa::NfaVertexRef, Nfa};

pub struct StateVertex {
    pub acceptable: bool,
    pub epsilon_closure: Vec<NfaVertexRef>,
    pub neighbors: HashMap<u8, DfaVertexRef>,
}

pub struct DfaVertexRef(Rc<RefCell<StateVertex>>);

impl DfaVertexRef {
    pub fn new() -> Self {
        DfaVertexRef(Rc::new(RefCell::new(StateVertex {
            acceptable: false,
            epsilon_closure: Vec::new(),
            neighbors: HashMap::new(),
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

pub struct Dfa {
    pub vertexs: Vec<DfaVertexRef>,
    pub lookup: Vec<HashMap<u8, usize>>,
}

impl Dfa {
    pub fn get_start(&self) -> DfaVertexRef {
        DfaVertexRef::clone(&self.vertexs[0])
    }

    pub fn is_acceptable(&self, id: usize) -> bool {
        self.vertexs.get(id).unwrap().borrow().acceptable
    }
}

/// epsilon_closure
/// get epsilon closure of a vertex
/// result will be stored in visited
pub fn epsilon_closure(vertex: NfaVertexRef, visited: &mut Vec<NfaVertexRef>) {
    if visited.contains(&vertex) {
        return;
    }

    // 将自身添加到闭包中
    visited.push(NfaVertexRef::clone(&vertex));

    // 遍历相邻节点
    vertex.borrow().epsilon_neighbors.iter().for_each(|each| {
        epsilon_closure(NfaVertexRef::clone(&each), visited);
    });
}

/// to_dfa
/// convert nfa to dfa
/// return start vertex of dfa
pub fn to_dfa(nfa: &Nfa) -> Dfa {
    // 以nfa的开始节点的epsilon-closure为开始节点
    let start = DfaVertexRef::new();

    epsilon_closure(
        NfaVertexRef::clone(&nfa.start),
        &mut start.borrow_mut().epsilon_closure,
    );

    // 递归扩展
    let mut visited = Vec::new();
    tarverse_vertex(DfaVertexRef::clone(&start), &mut visited);

    // 标注可接受状态
    visited.iter().for_each(|each| {
        if each.borrow().epsilon_closure.contains(&nfa.end) {
            each.borrow_mut().acceptable = true;
        }
    });

    let mut lookup = Vec::new();
    // 生成lookup table
    visited.iter().for_each(|each| {
        let mut curr_vertex = HashMap::new();

        // 遍历所有的邻居，写到转换表中
        each.borrow()
            .neighbors
            .iter()
            .for_each(|(&cond, neighbor)| {
                let id = visited
                    .iter()
                    .position(|each| (*each) == (*neighbor))
                    .unwrap();
                curr_vertex.insert(cond, id);
            });

        lookup.push(curr_vertex);
    });

    Dfa {
        vertexs: visited,
        lookup,
    }
}

// 扩展当前dfa节点
pub fn tarverse_vertex(vertex: DfaVertexRef, visited: &mut Vec<DfaVertexRef>) {
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
                    epsilon_closure(NfaVertexRef::clone(&nfa_vertex), &mut closure);

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
    vertex
        .borrow()
        .neighbors
        .iter()
        .for_each(|(_, neighbor)| tarverse_vertex(DfaVertexRef::clone(neighbor), visited));
}
