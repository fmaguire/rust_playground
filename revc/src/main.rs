use std::env;
use std::fs;
use std::collections;

fn main() {

    let args: Vec<String> = env::args().collect();

    let dna = fs::read_to_string(&args[1])
                    .expect("Failed to read file");

    let mut revc_translate = collections::HashMap::new();
    revc_translate.insert('G', 'C');
    revc_translate.insert('C', 'G');
    revc_translate.insert('T', 'A');
    revc_translate.insert('A', 'T');
    
    let mut dna_c = String::new();

    for base in dna.chars() {
        match revc_translate.get(&base) {
            Some(&revc_base) => dna_c.push(revc_base),
            None => (),
        }
    }
    
    let dna_revc: String = dna_c.chars().rev().collect();
    println!("{}", dna_revc);
}
