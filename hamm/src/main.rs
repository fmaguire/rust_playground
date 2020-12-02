use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let dna = input.lines().collect::<Vec<&str>>();

    let hamming = dna[0].chars()
                        .zip(dna[1].chars())
                        .filter(|(base_0, base_1)| base_0 != base_1)
                        .count();

    println!("{}", hamming);

    Ok(())
}
