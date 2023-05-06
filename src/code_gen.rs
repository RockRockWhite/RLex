use crate::LookupTable;

pub fn gen_code(
    declarations: &str,
    variables: &str,
    lookup_tables: &Vec<LookupTable>,
    handler_funcs: &Vec<String>,
) -> String {
    let lookup_table_json = format!("r#\"{}\"#", serde_json::to_string(lookup_tables).unwrap());
    // 生成handler_funcs
    let mut handler_funcs_str = String::new();
    for each in handler_funcs {
        handler_funcs_str.push_str(&format!("\t\thandler_funcs.push(Box::new({}));\n", each));
    }

    format!(
        r#"
use serde::{{Deserialize, Serialize}};
use std::collections::HashMap;

// declarations
// ======================
{declarations}
// ======================

#[derive(Serialize, Deserialize)]
pub struct LookupTable {{
    pub accept_states: Vec<usize>,
    pub states: Vec<HashMap<u8, usize>>,
}}

impl LookupTable {{
    pub fn new() -> Self {{
        LookupTable {{
            accept_states: Vec::new(),
            states: Vec::new(),
        }}
    }}

    pub fn is_acceptable(&self, id: usize) -> bool {{
        self.accept_states.contains(&id)
    }}
}}

pub struct Rlex {{
    // variables
    // ======================
    {variables}
    // ======================
}}

impl Rlex {{
    pub fn lex(&self, s: &str) {{
        // 词法分析
        // 生成lookup_table
        let lookup_table:Vec<LookupTable>= serde_json::from_str({lookup_table_json}).unwrap();
        let mut handler_funcs: Vec<Box<dyn Fn(&str) -> ()>> = Vec::new();

        // rules
        // ======================
{handler_funcs_str}
        // ======================

        // 执行词法分析
        let mut match_str: &str = "";
        let mut res_str = s;

        while res_str != "" {{
            for (id, each) in lookup_table.iter().enumerate() {{
                (match_str, res_str) = Self::match_reg(res_str, each);

                // 如果匹配成功,则执行对应的handler
                if match_str != "" {{
                    // 执行handler_func
                    handler_funcs.get(id).unwrap()(match_str);
                    break;
                }}
            }}

            // 未知错误
            if match_str == "" {{
                println!("unknown error:{{}}", &res_str[..1]);
                res_str = &res_str[1..];
            }}
        }}
    }}

    /// match_reg
    /// for match a sub string that match a reg
    /// return (matched_str, rest_str)
    fn match_reg<'a>(s: &'a str, lookup: &LookupTable) -> (&'a str, &'a str) {{
        let mut state: usize = 0;
        let mut last_match_index = 0;
        let mut matched = false;

        for (index, each) in s.as_bytes().iter().enumerate() {{
            if let Some(next_state) = lookup.states[state].get(each) {{
                state = *next_state;

                // 如果可接受，则更新最后一个可接受状态
                if lookup.is_acceptable(state) {{
                    last_match_index = index;
                    matched = true;
                }}
            }} else {{
                break;
            }}
        }}

        if !matched {{
            return ("", s);
        }}
        (&s[..last_match_index + 1], &s[last_match_index + 1..])
    }}
}}
    "#
    )
    .to_string()
}
