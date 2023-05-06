use std::env;

use rlex::gen_code;

fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: rlex <config_file> <output_file>");
        return;
    }

    let config = rlex::parse_config(&args[1]);

    // 生成lookup_table和handler_funcs
    let mut lookup_tables = Vec::new();
    let mut handler_funcs = Vec::new();
    for (reg, handler_func) in config.rules.iter() {
        handler_funcs.push(handler_func.clone());
        let postfix = rlex::to_postfix(reg);
        let nfa = rlex::to_nfa(&postfix);
        let dfa = rlex::to_dfa(&nfa);
        lookup_tables.push(dfa.lookup_table);
    }

    // 生成代码
    let code = gen_code(
        &config.declarations,
        &config.variables,
        &lookup_tables,
        &handler_funcs,
    );

    // 写入文件
    std::fs::write(&args[2], code).unwrap();
}

// fn my_test() {
//     let mut reg_vec = Vec::new();

//     let number = rlex::to_postfix("(0|1|2|3|4)*");
//     let number = rlex::to_nfa(&number);
//     let number = rlex::to_dfa(&number);
//     reg_vec.push(number.lookup_table);

//     let identifier = rlex::to_postfix("(a|b|c|d|e)(a|b|c|d|e|0|1|2|3|4)*");
//     let identifier = rlex::to_nfa(&identifier);
//     let identifier = rlex::to_dfa(&identifier);
//     reg_vec.push(identifier.lookup_table);

//     let error = rlex::to_postfix("( |;|?|,|!|=)*");
//     let error = rlex::to_nfa(&error);
//     let error = rlex::to_dfa(&error);
//     reg_vec.push(error.lookup_table);

//     let mut handler_funcs = Vec::new();
//     handler_funcs.push("|s|{ println!(\"number: {}\", s); }".to_string());
//     handler_funcs.push("|s|{ println!(\"identifier: {}\", s); }".to_string());
//     handler_funcs.push("|s|{ println!(\"error: {}\", s); }".to_string());

//     println!("res_str: {}", gen_code("", "", &reg_vec, &handler_funcs));
// }

// /// match_reg
// /// 用于匹配正则表达式
// fn match_reg<'a>(s: &'a str, lookup: &rlex::LookupTable) -> (&'a str, &'a str) {
//     let mut state: usize = 0;
//     let mut last_match_index = 0;
//     let mut matched = false;

//     for (index, each) in s.as_bytes().iter().enumerate() {
//         if let Some(next_state) = lookup.states[state].get(each) {
//             state = *next_state;

//             // 如果可接受，则更新最后一个可接受状态
//             if lookup.is_acceptable(state) {
//                 last_match_index = index;
//                 matched = true;
//             }
//         } else {
//             break;
//         }
//     }

//     if !matched {
//         return ("", s);
//     }
//     (&s[..last_match_index + 1], &s[last_match_index + 1..])
// }
