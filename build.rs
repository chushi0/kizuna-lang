use lalrpop;

fn main() {
    println!("cargo::rerun-if-changed=./src/compiler/grammar.lalrpop");
    lalrpop::process_root().unwrap();
}
