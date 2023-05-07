use rlex::gen_code;
use std::{env, error::Error};

fn main() {
    // 读取命令行参数
    let args = Args::build(env::args().collect()).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    run(args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    })
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // 读取配置文件
    let config = rlex::parse_config(&args.config_file)?;
    println!("{:?}", config.rules);

    // 生成lookup_table和handler_funcs
    let mut lookup_tables = Vec::new();
    let mut handler_funcs = Vec::new();
    for (reg, handler_func) in config.rules.iter() {
        handler_funcs.push(handler_func.clone());

        let re = rlex::RegexExpr::build(reg)?;
        let nfa = rlex::Nfa::build(&re);
        let dfa = rlex::Dfa::build(&nfa);
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
    std::fs::write(&args.output_file, code)?;
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
