use crate::engine::lexer;
use crate::engine::parser;
use crate::engine::codegen;

pub mod engine;

fn main() {
let plant = lexer::hello();
let plant = parser::hello();
let plant = codegen::hello();
}
