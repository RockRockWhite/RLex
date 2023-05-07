use regex::Regex;
use std::{collections::HashMap, error::Error, fs::File, io::Read};

const RLEX_FILE_SAMPLE: &str = r#"
%{
    // declarations
    // declare your structs here
    pub struct Foo {
        x: i32,
        y: i32,
    }

    pub struct Bar {
        x: i32,
        y: i32,
    }
%}
    // definitions
    // define your regex variables here
    number = "[0-9]*"
    idenfitier = "[A-Za-z][A-Za-z0-9]*"
    error = "( |;|?|,|!|=)*"
%%
    // rules
    // define your rules here
    // regex -> handler_func
    {number} -> |s|{
        println!("number: {}", s);
    } ;;
    {idenfitier} -> |s|{
        println!("idenfitier: {}", s);
    } ;;
    {error} -> |s|{
        println!("error: {}", s);
    } ;;
%%
    // variables
    // define your variables here
    pub a: i32,
    pub b: i64, 
"#;

pub struct Config {
    pub declarations: String,
    pub rules: Vec<(String, String)>,
    pub variables: String,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    // 读取文件
    let (declarations, definitions, rules, variables) = read_config_file(path)?;
    // 将definitions中的变量提取出来
    let definitions = parse_definations(&definitions)?;
    // 将rules中的变量提取出来
    let rules = parse_rules(&rules, &definitions)?;

    Ok(Config {
        declarations,
        rules,
        variables,
    })
}

fn read_config_file(path: &str) -> Result<(String, String, String, String), Box<dyn Error>> {
    let mut f = File::open(path)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // 读取各项配置
    if let Some(captures) = Regex::new(r"(?s)%\{(.+?)%\}(.*?)%%(.*?)%%(.+)")
        .unwrap()
        .captures(&buf)
    {
        let declarations = captures[1].to_string();
        let definitions = captures[2].to_string();
        let rules = captures[3].to_string();
        let variables = captures[4].to_string();

        Ok((declarations, definitions, rules, variables))
    } else {
        Err(format!(
            "parsing config error: config file format error\n*.flex file sample:\n{}\n",
            RLEX_FILE_SAMPLE
        )
        .into())
    }
}

fn parse_definations(definitions: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut res: HashMap<String, String> = HashMap::new();

    // parse definitions to hash_map
    for captures in Regex::new(r#"(\w+)\s*=\s*\"(.*?)\""#)
        .unwrap()
        .captures_iter(&definitions)
    {
        let key = captures[1].trim().to_string();
        let value = captures[2].trim().to_string();
        res.insert(key, value);
    }

    // replace all variables
    let old_difinations = res.clone();
    for (_, value) in &mut res {
        *value = replace_regex_variables(value, &old_difinations)?;
    }

    Ok(res)
}

fn parse_rules(
    rules: &str,
    definitions: &HashMap<String, String>,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut res = Vec::new();
    for captures in Regex::new(r#"([\s\S]*?)->([\s\S]*?);;"#)
        .unwrap()
        .captures_iter(&rules)
    {
        let key = captures.get(1).unwrap().as_str().trim().to_string();
        // 替换key中变量
        let key = replace_regex_variables(&key, &definitions)?;
        let value = captures.get(2).unwrap().as_str().trim().to_string();
        res.push((key, value));
    }

    Ok(res)
}

fn replace_regex_variables(
    s: &str,
    definitions: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut res = s.to_string();
    let mut variables = Vec::new();

    // find all variables
    Regex::new(r"\{\w+\}")
        .unwrap()
        .captures_iter(&res)
        .for_each(|captures| {
            variables.push(captures[0].to_string());
        });

    // replace all variables
    for variable in variables {
        let variable_no_blanket = variable.replace("{", "").replace("}", "");
        if let Some(val) = definitions.get(&variable_no_blanket) {
            res = res.replace(&variable, format!("({})", val).as_str());
        } else {
            return Err(format!(
                "parsing config error: variable \"{}\" not defined",
                variable_no_blanket
            )
            .into());
        }
    }
    Ok(res)
}
