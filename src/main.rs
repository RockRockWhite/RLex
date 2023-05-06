use core::num;

fn main() {
    // let postfix = rlex::to_postfix("c(a|bbcb*)*(ab)");
    // // let postfix = rlex::to_postfix("c(abc|b*)");
    // let nfa = rlex::to_nfa(&postfix);
    // let dfa = rlex::to_dfa(&nfa);

    // // let mermaid = rlex::mermaid::parse_nfa(&nfa);
    // let mermaid = rlex::mermaid::parse_dfa(&dfa);
    // println!("{}", mermaid);

    // let lookup = &dfa.lookup;

    // lookup.iter().enumerate().for_each(|(index, each)| {
    //     print!("{}: [ ", index);
    //     each.iter().for_each(|(&key, value)| {
    //         print!("{}: {:?} ", key as char, value);
    //     });
    //     println!(" ]");
    // });
    // println!("is_acceptable: {:?}", match_reg("caaabbcbbcbab", &dfa));
    my_test()
}

fn my_test() {
    let mut reg_vec = Vec::new();

    let number = rlex::to_postfix("(0|1|2|3|4)*");
    let number = rlex::to_nfa(&number);
    let number = rlex::to_dfa(&number);
    reg_vec.push(number);

    let identifier = rlex::to_postfix("(a|b|c|d|e)(a|b|c|d|e|0|1|2|3|4)*");
    let identifier = rlex::to_nfa(&identifier);
    let identifier = rlex::to_dfa(&identifier);
    reg_vec.push(identifier);

    let error = rlex::to_postfix("( |;|?|,|!|=)*");
    let error = rlex::to_nfa(&error);
    let error = rlex::to_dfa(&error);
    reg_vec.push(error);

    let mut match_str: &str = "";
    let mut res_str = "abcde1234123 41234 1011 ===6666";
    while res_str != "" {
        for (id, each) in reg_vec.iter().enumerate() {
            (match_str, res_str) = match_reg(res_str, each);
            if (match_str != "") {
                println!("{}: {}", id, match_str);
                break;
            }
        }

        // 未知错误
        println!("unknown error:{}", &res_str[..1]);
        res_str = &res_str[1..];
    }
}

/// match_reg
/// 用于匹配正则表达式
fn match_reg<'a>(s: &'a str, dfa: &rlex::Dfa) -> (&'a str, &'a str) {
    let mut state: usize = 0;
    let mut last_match_index = 0;
    let mut matched = false;

    for (index, each) in s.as_bytes().iter().enumerate() {
        if let Some(next_state) = dfa.lookup[state].get(each) {
            state = *next_state;

            // 如果可接受，则更新最后一个可接受状态
            if dfa.is_acceptable(state) {
                last_match_index = index;
                matched = true;
            }
        } else {
            break;
        }
    }

    if !matched {
        return ("", s);
    }
    (&s[..last_match_index + 1], &s[last_match_index + 1..])
}
