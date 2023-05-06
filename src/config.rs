use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

pub struct Config {
    pub declarations: String,
    pub rules: Vec<(String, String)>,
    pub variables: String,
}

pub fn parse_config(path: &str) -> Config {
    // 读取文件
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    // 读取各项配置
    let captures = Regex::new(r"(?s)%\{(.+?)%\}(.*?)%%(.*?)%%(.+)")
        .unwrap()
        .captures(&buf)
        .unwrap();
    let declarations = captures[1].to_string();
    let definitions = captures[2].to_string();
    let rules = captures[3].to_string();
    let variables = captures[4].to_string();

    // 将definitions中的变量提取出来
    let mut definitions_map: HashMap<String, String> = HashMap::new();
    for captures in Regex::new(r#"(\w+)\s*=\s*\"(.*?)\""#)
        .unwrap()
        .captures_iter(&definitions)
    {
        let key = captures[1].trim().to_string();
        let value = captures[2].trim().to_string();
        definitions_map.insert(key, value);
    }

    // 将rules中的变量提取出来
    let mut rules_vec = Vec::new();
    for captures in Regex::new(r#"([\s\S]*?)->([\s\S]*?);;"#)
        .unwrap()
        .captures_iter(&rules)
    {
        let key = captures.get(1).unwrap().as_str().trim().to_string();
        let value = captures.get(2).unwrap().as_str().trim().to_string();
        rules_vec.push((key, value));
    }

    // 替换rules_vec中key的变量
    for (key, _) in &mut rules_vec {
        let mut to_replace = Vec::new();

        Regex::new(r"\{\w+\}")
            .unwrap()
            .captures_iter(key)
            .for_each(|captures| {
                to_replace.push(captures[0].to_string());
            });

        for each in to_replace {
            *key = key.replace(
                &each,
                definitions_map
                    .get(&each.replace("{", "").replace("}", ""))
                    .unwrap(),
            );
        }
    }

    Config {
        declarations,
        rules: rules_vec,
        variables,
    }
}
