use rlex::gen_code;
use std::{env, error::Error};

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
    let mut handler_funcs = Vec::new();

    let mut nfa_builder = rlex::NfaBuilder::new();
    // build nfa
    for (reg, handler_func) in config.rules.iter() {
        let handler_id = handler_funcs.len();
        handler_funcs.push(handler_func.clone());

        nfa_builder.add_rule(&rlex::RegexExpr::build(reg)?, handler_id);
    }
    let nfa = nfa_builder.build().unwrap();
    let dfa = rlex::Dfa::build(&nfa);
    let lookup_table = dfa.lookup_table;

    // 生成代码
    let code = gen_code(
        &config.declarations,
        &config.variables,
        &lookup_table,
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
