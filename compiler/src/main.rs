use clap::{ArgAction, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum EmitType {
    Ast,
    Asm,
    LlvmIr,
    Object,
}

#[derive(Parser, Debug)]
#[command(name = "compiler", version, about = "My compiler")]
struct Cli {
    /// Input source file!
    input: String,

    /// Output file
    #[arg(short, long, default_value = "a.out")]
    output: String,

    #[arg(long, value_enum, default_value_t = EmitType::Asm)]
    emit: EmitType,

    /// Enable
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    println!("Input: {}", args.input);
    println!("Output: {}", args.output);
    println!("Verbose: {}", args.verbose);
    println!("Emit: {:?}", args.emit);
    println!("Hello, world!");
    // Standard suffix to indicate u64 type
    let a = lexer::add(1_u64, 2_u64);

    // 玩嘢，3.0_f64 use _f64 to make 3.0 an f64 then explicitly type cast as u64
    let b = parser::add(3.0_f64 as u64, 4.0_f32 as u64);

    // Just type cast to u64 using as u64
    let c = codegen::add(5.0 as u64, 6.0 as u64);

    println!("a={}, b={} and c={}", a, b, c);
}
