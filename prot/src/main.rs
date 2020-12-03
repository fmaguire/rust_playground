use std::collections;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_seq = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let codon_table = "UUU F      CUU L      AUU I      GUU V
UUC F      CUC L      AUC I      GUC V
UUA L      CUA L      AUA I      GUA V
UUG L      CUG L      AUG M      GUG V
UCU S      CCU P      ACU T      GCU A
UCC S      CCC P      ACC T      GCC A
UCA S      CCA P      ACA T      GCA A
UCG S      CCG P      ACG T      GCG A
UAU Y      CAU H      AAU N      GAU D
UAC Y      CAC H      AAC N      GAC D
UAA Stop   CAA Q      AAA K      GAA E
UAG Stop   CAG Q      AAG K      GAG E
UGU C      CGU R      AGU S      GGU G
UGC C      CGC R      AGC S      GGC G
UGA Stop   CGA R      AGA R      GGA G
UGG W      CGG R      AGG R      GGG G"
        .split_whitespace()
        .collect::<Vec<_>>();

    let mut rna_translate = collections::HashMap::new();
    for ix in (0..codon_table.len()).step_by(2) {
        rna_translate.insert(codon_table[ix].to_string(), codon_table[ix + 1].to_string());
    }

    let mut protein = Vec::new();
    for ix in (0..input_seq.len() - 3).step_by(3) {
        let codon = input_seq[ix..ix + 3].to_string();

        match rna_translate.get(&codon) {
            Some(aa) => protein.push(aa),
            None => (),
        }
    }

    for trans_codon in protein.into_iter() {
        match trans_codon.as_str()  {
            "Stop" => break,
            _ => print!("{}", trans_codon),
        }
    }
    println!();
}
