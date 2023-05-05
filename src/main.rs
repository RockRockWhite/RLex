use std::rc::Rc;

fn main() {
    let postfix = rlex::to_postfix("c(a|bbcb*)*(ab)");
    let nfa = rlex::to_nfa(&postfix);
    let mermaid = rlex::mermaid::parse_nfa(&nfa);
    println!("{}", mermaid);

    let dfa = rlex::to_dfa(&nfa);

    let res = dfa;

    // let mut closure = Vec::new();
    // rlex::epsilon_closure(Rc::clone(&nfa.start.borrow().neighbors[&b'c']), &mut closure);

    // for each in closure {
    //     println!("{}", each.as_ptr() as usize);
    // }
}
