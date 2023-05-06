fn main() {
    // let postfix = rlex::to_postfix("c(a|bbcb*)*(ab)");
    let postfix = rlex::to_postfix("c(abc|b*)");
    let nfa = rlex::to_nfa(&postfix);
    let dfa = rlex::to_dfa(&nfa);

    // let mermaid = rlex::mermaid::parse_nfa(&nfa);
    let mermaid = rlex::mermaid::parse_dfa(&dfa);
    println!("{}", mermaid);

    let lookup = dfa.lookup;

    lookup.iter().enumerate().for_each(|(index, each)| {
        print!("{}: [ ", index);
        each.iter().for_each(|(&key, value)| {
            print!("{}: {:?} ", key as char, value);
        });
        println!(" ]");
    });
}
