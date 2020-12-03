use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;
    let dna = file.lines().collect::<Vec<&str>>();

    let input_len = dna[0].len();
    let motif_len = dna[1].len();

    for n in 0..input_len {
        if (n + motif_len) < input_len {
            if &dna[0][n..n + motif_len] == dna[1] {
                print!("{:?} ", n + 1);
            }
        }

    }

    
    Ok(())
}
