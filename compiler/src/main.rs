fn main() {
    println!("Hello, world!");
    // Standard suffix to indicate u64 type
    let a = lexer::add(1_u64, 2_u64);

    // 玩嘢，3.0_f64 use _f64 to make 3.0 an f64 then explicitly type cast as u64
    let b = parser::add(3.0_f64 as u64, 4.0_f32 as u64);

    // Just type cast to u64 using as u64
    let c = codegen::add(5.0 as u64, 6.0 as u64);

    println!("a={}, b={} and c={}", a, b, c);
}
