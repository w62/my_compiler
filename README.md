This is the directory containing my progression on Writing a C Compiler.

29-10-2026 Task1 

Write a compiler driver takes the following options

--lex  lexer 
--parse parser
--codegen code generator 

31-10-2026

Crates and modules. 
The three components are broken down into three explicitly crates. 
- compiler is the main program
- lex, parser and codegen are three separate crates called by compiler.

Types
1_u64 to explicitly convert 1 to type u64.
