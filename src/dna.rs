use super::{Nucleotide,Adenine, Cytosine, Guanine, Thymine};

pub struct NucleoCounter{
    pub a: u32,
    pub c: u32,
    pub g: u32,
    pub t: u32,
}

impl NucleoCounter {
    pub fn make() -> NucleoCounter {
        NucleoCounter {
            a: 0,
            c: 0,
            g: 0,
            t: 0
        }
    }

    pub fn add_nucl(&mut self, nucl_type: Nucleotide) {
        match nucl_type {
            Adenine  => self.a += 1,
            Cytosine => self.c += 1,
            Guanine  => self.g += 1,
            Thymine  => self.t += 1
        }
    }

    pub fn to_simple_str(&self) -> ~str {
        let s = format!("{:u} {:u} {:u} {:u}",
                    self.a,
                    self.c,
                    self.g,
                    self.t);
        s
    }
}

pub fn count_nucleos(data: &str) -> NucleoCounter {
    let mut nucl = NucleoCounter::make();
    for chr in data.chars() {
        if chr == 'A' {
            nucl.add_nucl(Adenine);
        } else if chr == 'C' {
            nucl.add_nucl(Cytosine);
        } else if chr == 'G' {
            nucl.add_nucl(Guanine);
        } else if chr == 'T' {
            nucl.add_nucl(Thymine);
        }
    }
    nucl
}

fn main() {

}

#[cfg(test)]
mod test {

    use super::count_nucleos;

    #[test]
    fn test_1(){
        let raw = &"AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let data = count_nucleos(raw);

        assert_eq!(data.a, 20);
        assert_eq!(data.c, 12);
        assert_eq!(data.g, 17);
        assert_eq!(data.t, 21);

    }

        #[test]
    fn test_2(){
        let raw = &"ATGCACCGCTAGAGCATGATTTAATTTATATTCAGGCGTAATGTGAAACAATCTCAACCCTCTGGTGCGTATATTCTCATGGTGAGTTCAGAGCTGTGGCTCTGTACTCAAGAGCGGGGGTATACCGGGGTGCCTCAAACTTTGCGTCAATTCGTTATGTTGGGATAGAGGGTGGACCCCAGCCTAGGAAATGATTATCTCATTGTTGACGTGATTCACCGGGGCCTATTATTCGAGATCGATGATAGCAACTCCCGGTATAAGGAATCTAATTCGCCGTATGTTGATGCGAAGTGCGGGGTTGATTAGACTCTCGTTCGCGTACGGGGTCGTCAATTATACGCGCAAACAGTTTCCGCCTCATTGATCGCCGAACAGTTGTGTTATAATTCGGCGCAGCTTTTTGAATAAACTTATACAGATTTGCTGCCACCGTTACGCAAGGTGGCCTGATTTCAACTCTGACCATGTTGCAGGGATAGGGCGGTCGATGTGCGACCCTCGTGCCCTGGATACGGCACGGCTATTAGGCCTGTAAGAACCACGACACGAATCTTTCCTGACTTGCAAAGATACCATAAGCCATTCACTGCCACAGGGAGGTAGAACGTGTTAATGAGCATCAGTCGTAGGAGGCACGAAAACGCTTAGACCCCTTTTTGTATGAAGTACATAATCAATACAGATTGGGGAACCTACGCGGAAATGCTTAGGTGGCTCCGCGCAGATTCCACCACGGCTTGAACCGTATGTGCCGATTCTGGCGATTCTCTGAGCTGAACAGGGCACGCGGCTGGAACCATAGCCAGCAGCGCAATTGACATCGACCCTGAGAAGCGGATACTAATGTTTACAATAAAGTACATCGAGGGGGTGAGCAGGCAAGTGTCCTGATGCATGTAA";
        let data = count_nucleos(raw);

        assert_eq!(data.a, 230);
        assert_eq!(data.c, 204);
        assert_eq!(data.g, 238);
        assert_eq!(data.t, 231);

    }

}