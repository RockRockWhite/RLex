fn main() {
    // let postfix = rlex::to_postfix("c(a|bbcb*)*(ab)");
    let postfix = rlex::to_postfix("c(abc|b*)");
    let nfa = rlex::to_nfa(&postfix);
    let dfa = rlex::to_dfa(&nfa);

    // let mermaid = rlex::mermaid::parse_nfa(&nfa);
    let mermaid = rlex::mermaid::parse_dfa(&dfa);
    println!("{}", mermaid);

    let lookup = &dfa.lookup;

    lookup.iter().enumerate().for_each(|(index, each)| {
        print!("{}: [ ", index);
        each.iter().for_each(|(&key, value)| {
            print!("{}: {:?} ", key as char, value);
        });
        println!(" ]");
    });
    println!("is_acceptable: {:?}", match_reg("cbbbbbbbbb", &dfa));
}

fn match_reg(s: &str, dfa: &rlex::Dfa) -> bool {
    let mut state = 0;

    for each in s.as_bytes() {
        if let Some(next_state) = dfa.lookup[state].get(each) {
            state = *next_state;
        } else {
            return false;
        }
    }

    dfa.is_acceptable(state)
}
