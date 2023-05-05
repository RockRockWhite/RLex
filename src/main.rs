fn main() {
    let postfix = rlex::to_postfix("(c(abc|b*))");

    // for each in postfix.0 {
    //     print!("{}", each as char);
    // }

    let nfa = rlex::to_nfa(postfix);
    let mermaid = rlex::mermaid::parse_nfa(nfa);

    println!("{}", mermaid);

    // let nfa = rlex::to_nfa(postfix);
    // nfa::to_nfa();
    println!("Hello, world!");
}
