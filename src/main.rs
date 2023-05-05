fn main() {
    // let postfix = rlex::to_postfix("c(a|bbcb*)*(ab)");
    let postfix = rlex::to_postfix("c(abc|b*)");
    let nfa = rlex::to_nfa(&postfix);
    let mermaid = rlex::mermaid::parse_nfa(&nfa);
    println!("{}", mermaid);

    rlex::to_dfa(&nfa);
}
