pub mod compiler;
pub mod runtime;
use anyhow::{anyhow, Result};
use compiler::ast;
use runtime::{scope::Value, vm::VM};
use snap::{read::FrameDecoder, write::FrameEncoder};
use std::{
    env::{self, Args},
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
    let mut args = env::args();

    args.next();

    match args.next() {
        Some(s) => {
            if s == "build" {
                build_scripts(args);
            } else if s == "run" {
                run_scripts(args);
            } else if s == "help" {
                print_help();
            } else {
                eprintln!("unknown action `{}`", s);
                print_help();
            }
        }
        None => print_help(),
    }
}

fn print_help() {
    println!("\tbuild - Build scripts and save binary file");
    println!("\trun - Run binary file");
    println!("\thelp - print this message");

    std::process::exit(0);
}

fn build_scripts(args: Args) {
    for file in args {
        let path = Path::new(&file);
        let dir = path.with_file_name(
            (path.file_stem().unwrap_or_default().to_string_lossy() + ".kb").as_ref(),
        );

        if let Err(e) = compile_and_save(&file, dir.to_string_lossy().as_ref()) {
            eprintln!("{file}: {e}");
        };
    }
}

fn run_scripts(args: Args) {
    let vm = init_vm();
    for file in args {
        let Ok(script) = load_from_binary(&file) else {
            eprintln!("Failed to load script: {file}");
            continue;
        };
        vm.submit_script(script);
    }
}

fn compile_and_save(input: &str, output: &str) -> Result<()> {
    let mut buf = String::new();
    File::open(input)?.read_to_string(&mut buf)?;
    let compile_result = compiler::parser::parse(&buf).map_err(|e| anyhow!("{e}"))?;
    let encode = bincode::serialize(&compile_result)?;
    FrameEncoder::new(File::create(output)?).write_all(&encode)?;
    Ok(())
}

fn load_from_binary(file: &str) -> Result<ast::Script> {
    let mut buf = Vec::new();
    FrameDecoder::new(File::open(file)?).read_to_end(&mut buf)?;
    Ok(bincode::deserialize(&buf)?)
}

fn init_vm() -> VM {
    let vm = VM::new();

    let scope = vm.global_scope();
    let mut scope = scope.0.write().unwrap();
    scope.add_native_function("print".to_owned(), |_scope, params| {
        if params.is_empty() {
            print!("");
        } else {
            print!("{}", params[0].to_string());
        }

        Value::None
    });
    scope.add_native_function("println".to_owned(), |_scope, params| {
        if params.is_empty() {
            println!("");
        } else {
            println!("{}", params[0].to_string());
        }

        Value::None
    });

    vm
}
