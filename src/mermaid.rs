use std::{cell::RefCell, rc::Rc};

use crate::{dfa::Dfa, nfa::StateVertex, DfaVertexRef, NFA};

/// Converts an NFA to a mermaid graph
/// https://mermaid-js.github.io/mermaid/#/graph?id=graph
/// ```mermaid
/// graph LR
/// A((A))
/// B((B))
/// A--a-->B
/// ```
pub fn parse_nfa(nfa: &NFA) -> String {
    // 遍历图
    let mut visited = Vec::new();
    let edge = tarverse_nfa_vertex(Rc::clone(&nfa.start), &mut visited);

    // 添加节点
    let mut vertex = String::new();
    visited.iter().enumerate().for_each(|(id, each)| {
        // 特殊标记start end节点
        if Rc::ptr_eq(&nfa.start, each) {
            vertex.push_str(&format!("{}(({}))\n", id, "S"));
            return;
        }

        if Rc::ptr_eq(&nfa.end, each) {
            vertex.push_str(&format!("{}(({}))\n", id, "E"));
            return;
        }

        vertex.push_str(&format!("{}(({}))\n", id, id));
    });

    format!("graph LR\n{}\n{}", vertex, edge)
}

/// tarverse_vertex
/// 遍历NFA节点，返回其顶点和边
/// return edges
fn tarverse_nfa_vertex(
    start: Rc<RefCell<StateVertex>>,
    visited: &mut Vec<Rc<RefCell<StateVertex>>>,
) -> String {
    let mut edges = String::new();

    // 如果已经访问过，直接返回
    if visited.iter().any(|each| {
        if Rc::ptr_eq(&start, each) {
            return true;
        }
        false
    }) {
        return edges;
    }

    // 标记为已访问, 将vec的下标作为id
    let id = visited.len();
    visited.push(Rc::clone(&start));

    let find_vertex_id =
        |vertex: Rc<RefCell<StateVertex>>, visited: &Vec<Rc<RefCell<StateVertex>>>| -> usize {
            visited
                .iter()
                .enumerate()
                .find(|(_, each)| Rc::ptr_eq(&vertex, each))
                .unwrap()
                .0
        };

    // 遍历节点
    (*start)
        .borrow()
        .neighbors
        .iter()
        .for_each(|(&cond, vertex)| {
            let neighbor_edges = tarverse_nfa_vertex(Rc::clone(&vertex), visited);
            // 将结果添加到边中
            edges.push_str(&neighbor_edges);

            let neighbor_id = find_vertex_id(Rc::clone(&vertex), visited);

            // 添加当前节点到该节点的边
            edges.push_str(&format!("{}--{}-->{}\n", id, cond as char, neighbor_id));
        });

    // 遍历epsilon-move节点
    // 遍历节点
    (*start)
        .borrow()
        .epsilon_neighbors
        .iter()
        .for_each(|vertex| {
            let neighbor_edges = tarverse_nfa_vertex(Rc::clone(&vertex), visited);
            // 将结果添加到边中
            edges.push_str(&neighbor_edges);

            let neighbor_id = find_vertex_id(Rc::clone(&vertex), visited);

            // 添加当前节点到该节点的边
            edges.push_str(&format!("{}--{}-->{}\n", id, "ε", neighbor_id));
        });

    edges
}

/// Converts an DFA to a mermaid graph
/// https://mermaid-js.github.io/mermaid/#/graph?id=graph
/// ```mermaid
/// graph LR
/// A((A))
/// B((B))
/// A--a-->B
/// ```
/// **节点id仅用于标注，不保证与lookup table的id完全一样**
pub fn parse_dfa(dfa: &Dfa) -> String {
    // 遍历图
    let mut visited = Vec::new();
    let edge = tarverse_dfa_vertex(dfa.get_start(), &mut visited);

    // 添加节点
    let mut vertex = String::new();
    visited.iter().enumerate().for_each(|(id, each)| {
        if each.borrow().acceptable {
            vertex.push_str(&format!("{}(({}-A))\n", id, id));
        } else {
            vertex.push_str(&format!("{}(({}))\n", id, id));
        }
    });

    format!("graph LR\n{}\n{}", vertex, edge)
}

/// tarverse_dfa_vertex
/// 遍历dfa节点，返回其边
/// return edges
fn tarverse_dfa_vertex(vertex: DfaVertexRef, visited: &mut Vec<DfaVertexRef>) -> String {
    let mut edges = String::new();

    // 如果已经访问过，直接返回
    if visited.contains(&vertex) {
        return edges;
    }

    // 标记为已访问, 将vec的下标作为id
    let id = visited.len();
    visited.push(DfaVertexRef::clone(&vertex));

    let find_vertex_id = |vertex: DfaVertexRef, visited: &Vec<DfaVertexRef>| -> usize {
        visited
            .iter()
            .enumerate()
            .find(|(_, each)| (**each) == vertex)
            .unwrap()
            .0
    };

    // 遍历节点

    vertex
        .borrow()
        .neighbors
        .iter()
        .for_each(|(&cond, vertex)| {
            let neighbor_edges = tarverse_dfa_vertex(DfaVertexRef::clone(&vertex), visited);
            // 将结果添加到边中
            edges.push_str(&neighbor_edges);

            let neighbor_id = find_vertex_id(DfaVertexRef::clone(&vertex), visited);

            // 添加当前节点到该节点的边
            edges.push_str(&format!("{}--{}-->{}\n", id, cond as char, neighbor_id));
        });

    edges
}
