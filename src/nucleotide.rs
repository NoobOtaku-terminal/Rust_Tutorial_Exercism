use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if  !matches!(nucleotide,'A' | 'T' | 'C'|'G'){
        return Err(nucleotide);
    }
    // Validate the DNA string first
    if let Some(invalid) = dna.chars().find(|&c| !matches!(c, 'A' | 'T' | 'C' | 'G')) {
        return Err(invalid);
    }
    // Count occurrences
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    // Iterate over each character in the DNA string.
    // If an invalid character is found, return an error.
    for ch in dna.chars() {
        if !matches!(ch, 'A' | 'T' | 'C' | 'G') {
            return Err(ch);
        }
        // Increment the count for this nucleotide.
        // Using the entry API simplifies the logic.
        *counts.entry(ch).or_insert(0) += 1;
    }

    // Make sure all nucleotides are present in the map, even if their count is 0.
    for nuc in ['A', 'T', 'C', 'G'] {
        counts.entry(nuc).or_insert(0);
    }

    Ok(counts)
}
