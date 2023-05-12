use crate::LookupTable;

pub fn gen_code(
    declarations: &str,
    variables: &str,
    lookup_table: &LookupTable,
    handler_funcs: &Vec<String>,
) -> String {
    let lookup_table_json = format!("r#\"{}\"#", serde_json::to_string(lookup_table).unwrap());
    // 生成handler_funcs
    let mut handler_funcs_str = String::new();
    for each in handler_funcs {
        handler_funcs_str.push_str(&format!("\t\thandler_funcs.push(Box::new({}));\n", each));
    }

    format!(
        r#"
use serde::{{Deserialize, Serialize}};
use std::collections::{{HashMap, HashSet}};

// declarations
// ======================
{declarations}
// ======================

#[derive(Serialize, Deserialize)]
pub struct LookupState {{
    pub handlers: HashSet<usize>,
    pub neighbors: HashMap<u8, usize>,
}}

impl LookupState {{
    pub fn new() -> Self {{
        LookupState {{
            handlers: HashSet::new(),
            neighbors: HashMap::new(),
        }}
    }}

    pub fn insert_neighbor(&mut self, ch: u8, index: usize) {{
        self.neighbors.insert(ch, index);
    }}
}}

#[derive(Serialize, Deserialize)]
pub struct LookupTable {{
    pub states: Vec<LookupState>,
}}

impl LookupTable {{
    pub fn new() -> Self {{
        LookupTable {{ states: Vec::new() }}
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
        let lookup_table:LookupTable = serde_json::from_str({lookup_table_json}).unwrap();
        let mut handler_funcs: Vec<Box<dyn Fn(&str) -> ()>> = Vec::new();

        // rules
        // ======================
{handler_funcs_str}
        // ======================

        // 执行词法分析
        let mut match_str: &str;
        let mut res_str = s;
        let mut handler_id: usize;

        while res_str != "" {{
            (match_str, res_str, handler_id) = Self::match_reg(res_str, &lookup_table);

            // 如果匹配成功,则执行对应的handler
            if match_str != "" {{
                // 执行handler_func
                handler_funcs.get(handler_id).unwrap()(match_str);
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
    fn match_reg<'a>(s: &'a str, lookup: &LookupTable) -> (&'a str, &'a str, usize) {{
        let mut state: usize = 0;
        let mut last_match_index = 0;
        let mut matched = false;
        let mut handler_id = 0;

        for (index, each) in s.as_bytes().iter().enumerate() {{
            if let Some(next_state) = lookup.states[state].neighbors.get(each) {{
                state = *next_state;

                // 如果有handler, 则更新最后一个可接受状态
                if !lookup.states[state].handlers.is_empty() {{
                    last_match_index = index;
                    matched = true;
                    handler_id = *lookup.states[state].handlers.iter().min().unwrap();
                }}
            }} else {{
                break;
            }}
        }}

        if !matched {{
            return ("", s, handler_id);
        }}
        (
            &s[..last_match_index + 1],
            &s[last_match_index + 1..],
            handler_id,
        )
    }}
}}
    "#
    )
    .to_string()
}
