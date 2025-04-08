#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
    size: usize,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(index) = dna.chars().position(|c| !matches!(c, 'A' | 'T' | 'C' | 'G')) {
            return Err(index);
        }
        Ok(Dna {
            dna: dna.to_string(),
            size: dna.len(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self.dna.chars().map(|c| {
            match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => unreachable!(), // because of validation in Dna::new
            }
        }).collect();

        Rna {
            rna,
            size: self.size,
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(index) = rna.chars().position(|c| !matches!(c, 'A' | 'U' | 'C' | 'G')) {
            return Err(index);
        }
        Ok(Rna {
            rna: rna.to_string(),
            size: rna.len(),
        })
    }
}





