pub fn dna_to_rna_str(dna: &str ) -> ~str {
    std::str::replace(dna,"T", "U")
}



fn main() {
    let dna = ~"ACTG";
    let rna = dna_to_rna_str(dna);
    println!("DNA: {:}", dna);
    println!("RNA: {:}", rna);
}

#[cfg(test)]
mod test {
    use super::dna_to_rna_str;

    #[test]
    fn first_test(){
        let dna = ~"GATGGAACTTGACTACGTAAATT";
        assert_eq!(dna_to_rna_str(dna), ~"GAUGGAACUUGACUACGUAAAUU");
    }

}