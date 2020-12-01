use std::env;
use std::fs;
use std::collections;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dna = fs::read_to_string(&args[1])
                    .expect("Failed to read file");

    let mut rna_translate = collections::HashMap::new();
    rna_translate.insert('T', 'U');

    for base in dna.chars() {
        match rna_translate.get(&base) {
            Some(base_swap) => print!("{}", base_swap),
            None => print!("{}", base)
        }

    }

}
