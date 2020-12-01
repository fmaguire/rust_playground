use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let seq = &args[1];

    let a_count = seq.matches('A').count();
    let g_count = seq.matches('G').count();
    let c_count = seq.matches('C').count();
    let t_count = seq.matches('T').count();


    println!("{} {} {} {}", a_count, c_count, g_count, t_count);
}
