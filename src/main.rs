use rayon::{iter::Map, prelude::*};
use rlex::{gen_code, LookupTable};
use std::{collections::HashMap, env, error::Error};

fn main() {
    // let res = RegexExpr::to_charactors("a\\\\a");
    // println!("{:?}", res);
    // 读取命令行参数
    let args = Args::build(env::args().collect()).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    run(args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    println!(
        r#"Done.
Please add the following dependencies to your Cargo.toml:
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0""#
    );
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // 读取配置文件
    let config = rlex::parse_config(&args.config_file)?;

    // 生成lookup_table和handler_funcs
    // let mut lookup_tables = Vec::new();
    // let mut handler_funcs = Vec::new();

    // for (index, (reg, handler_func)) in config.rules.par_iter().enumerate() {
    //     handler_funcs.push(handler_func.clone());

    //     let re = rlex::RegexExpr::build(reg)?;
    //     let nfa = rlex::Nfa::build(&re);
    //     let dfa = rlex::Dfa::build(&nfa);
    //     lookup_tables.push(dfa.lookup_table);
    // }

    let vec: Vec<_> = config
        .rules
        .par_iter()
        .enumerate()
        .map(|(index, (reg, handler_func))| {
            let re = rlex::RegexExpr::build(reg).unwrap();
            let nfa = rlex::Nfa::build(&re);
            let dfa = rlex::Dfa::build(&nfa);

            (index, dfa.lookup_table, handler_func.clone())
        })
        .collect();

    let mut lookup_tables = Vec::with_capacity(vec.len());
    let mut handler_funcs = Vec::with_capacity(vec.len());

    // 初始化
    for _ in 0..vec.len() {
        lookup_tables.push(LookupTable::new());
        handler_funcs.push(String::new());
    }
    // 按照index生成新的vec
    for (index, lookup_table, handler_func) in vec {
        lookup_tables[index] = lookup_table;
        handler_funcs[index] = handler_func;
    }

    // 生成代码
    let code = gen_code(
        &config.declarations,
        &config.variables,
        &lookup_tables,
        &handler_funcs,
    );

    // 写入文件
    std::fs::write(&args.output_file, code)?;

    // 执行format
    if let Err(err) = std::process::Command::new("rustfmt")
        .arg(&args.output_file)
        .output()
    {
        return Err(format!("rustfmt error : {}", err).into());
    }

    Ok(())
}

struct Args {
    config_file: String,
    output_file: String,
}

impl Args {
    fn build(args: Vec<String>) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: rlex <config_file> <output_file>");
        }

        Ok(Args {
            config_file: args[1].clone(),
            output_file: args[2].clone(),
        })
    }
}
