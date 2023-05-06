use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// declarations
// ======================

pub struct Foo {
    x: i32,
    y: i32,
}

pub struct Bar {
    x: i32,
    y: i32,
}

// ======================

#[derive(Serialize, Deserialize)]
pub struct LookupTable {
    pub accept_states: Vec<usize>,
    pub states: Vec<HashMap<u8, usize>>,
}

impl LookupTable {
    pub fn new() -> Self {
        LookupTable {
            accept_states: Vec::new(),
            states: Vec::new(),
        }
    }

    pub fn is_acceptable(&self, id: usize) -> bool {
        self.accept_states.contains(&id)
    }
}

pub struct Rlex {
    // variables
    // ======================
    pub a: i32,
    pub b: i64,
    // ======================
}

impl Rlex {
    pub fn lex(&self, s: &str) {
        // 词法分析
        // 生成lookup_table
        let lookup_table:Vec<LookupTable>= serde_json::from_str(r#"[{"accept_states":[0,1,2,3,4,5],"states":[{"52":2,"51":5,"48":3,"50":1,"49":4},{"48":3,"50":1,"52":2,"49":4,"51":5},{"52":2,"49":4,"51":5,"50":1,"48":3},{"52":2,"49":4,"48":3,"51":5,"50":1},{"48":3,"52":2,"50":1,"51":5,"49":4},{"51":5,"48":3,"50":1,"49":4,"52":2}]},{"accept_states":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],"states":[{"97":15,"99":13,"100":12,"101":1,"98":14},{"97":11,"52":3,"100":4,"48":6,"98":8,"51":9,"101":2,"50":7,"49":10,"99":5},{"48":6,"99":5,"52":3,"100":4,"50":7,"49":10,"97":11,"98":8,"101":2,"51":9},{"52":3,"97":11,"50":7,"100":4,"98":8,"49":10,"99":5,"101":2,"51":9,"48":6},{"97":11,"51":9,"50":7,"52":3,"99":5,"48":6,"49":10,"100":4,"101":2,"98":8},{"101":2,"50":7,"52":3,"99":5,"98":8,"97":11,"100":4,"48":6,"51":9,"49":10},{"101":2,"48":6,"100":4,"52":3,"50":7,"97":11,"99":5,"51":9,"98":8,"49":10},{"49":10,"98":8,"50":7,"48":6,"52":3,"101":2,"51":9,"99":5,"100":4,"97":11},{"101":2,"98":8,"49":10,"52":3,"48":6,"51":9,"50":7,"97":11,"99":5,"100":4},{"48":6,"49":10,"100":4,"51":9,"98":8,"101":2,"52":3,"50":7,"97":11,"99":5},{"49":10,"50":7,"97":11,"51":9,"48":6,"99":5,"98":8,"52":3,"101":2,"100":4},{"52":3,"49":10,"51":9,"98":8,"48":6,"100":4,"101":2,"99":5,"97":11,"50":7},{"100":4,"50":7,"51":9,"49":10,"101":2,"52":3,"48":6,"98":8,"99":5,"97":11},{"52":3,"100":4,"48":6,"101":2,"49":10,"99":5,"50":7,"51":9,"98":8,"97":11},{"50":7,"97":11,"49":10,"101":2,"99":5,"48":6,"100":4,"52":3,"51":9,"98":8},{"51":9,"52":3,"101":2,"98":8,"48":6,"97":11,"50":7,"49":10,"100":4,"99":5}]},{"accept_states":[0,1,2,3,4,5,6],"states":[{"63":1,"61":6,"32":3,"44":2,"33":5,"59":4},{"59":4,"32":3,"33":5,"44":2,"63":1,"61":6},{"59":4,"32":3,"63":1,"33":5,"44":2,"61":6},{"61":6,"44":2,"59":4,"63":1,"32":3,"33":5},{"61":6,"33":5,"32":3,"59":4,"63":1,"44":2},{"61":6,"33":5,"44":2,"59":4,"63":1,"32":3},{"61":6,"44":2,"63":1,"59":4,"33":5,"32":3}]}]"#).unwrap();
        let mut handler_funcs: Vec<Box<dyn Fn(&str) -> ()>> = Vec::new();

        // rules
        // ======================
        handler_funcs.push(Box::new(|s| {
            println!("number: {}", s);
        }));
        handler_funcs.push(Box::new(|s| {
            println!("idenfitier: {}", s);
        }));
        handler_funcs.push(Box::new(|s| {
            println!("error: {}", s);
        }));

        // ======================

        // 执行词法分析
        let mut match_str: &str = "";
        let mut res_str = s;

        while res_str != "" {
            for (id, each) in lookup_table.iter().enumerate() {
                (match_str, res_str) = Self::match_reg(res_str, each);

                // 如果匹配成功,则执行对应的handler
                if match_str != "" {
                    // 执行handler_func
                    handler_funcs.get(id).unwrap()(match_str);
                    break;
                }
            }

            // 未知错误
            if match_str == "" {
                println!("unknown error:{}", &res_str[..1]);
                res_str = &res_str[1..];
            }
        }
    }

    /// match_reg
    /// for match a sub string that match a reg
    /// return (matched_str, rest_str)
    fn match_reg<'a>(s: &'a str, lookup: &LookupTable) -> (&'a str, &'a str) {
        let mut state: usize = 0;
        let mut last_match_index = 0;
        let mut matched = false;

        for (index, each) in s.as_bytes().iter().enumerate() {
            if let Some(next_state) = lookup.states[state].get(each) {
                state = *next_state;

                // 如果可接受，则更新最后一个可接受状态
                if lookup.is_acceptable(state) {
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
}
