use std::collections::{HashMap, HashSet};

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let cleaned = puzzle.replace("==", "=");
    let parts: Vec<&str> = cleaned.split('=').collect();
    let left = parts[0].trim();
    let right = parts[1].trim();

    let lhs_words: Vec<&str> = left.split('+').map(|w| w.trim()).collect();
    let rhs_word = right;

    let mut unique_letters = HashSet::new();
    let mut leading_letters = HashSet::new();

    for word in lhs_words.iter().chain(std::iter::once(&rhs_word)) {
        for (i, c) in word.chars().enumerate() {
            unique_letters.insert(c);
            if i == 0 && word.len() > 1 {
                leading_letters.insert(c);
            }
        }
    }

    if unique_letters.len() > 10 {
        return None;
    }

    let letters: Vec<char> = unique_letters.into_iter().collect();
    let mut assignments = HashMap::new();
    let mut used = [false; 10];

    fn word_value(word: &str, map: &HashMap<char, u8>) -> Option<u64> {
        let mut val = 0u64;
        for c in word.chars() {
            val = val * 10 + *map.get(&c)? as u64;
        }
        Some(val)
    }

    fn backtrack(
        idx: usize,
        letters: &Vec<char>,
        used: &mut [bool; 10],
        assignments: &mut HashMap<char, u8>,
        lhs: &Vec<&str>,
        rhs: &str,
        leading_letters: &HashSet<char>,
    ) -> bool {
        if idx == letters.len() {
            let left_sum: Option<u64> = lhs.iter().map(|w| word_value(w, assignments)).sum();
            let right_val = word_value(rhs, assignments);
            return left_sum.is_some() && right_val.is_some() && left_sum.unwrap() == right_val.unwrap();
        }

        let current_letter = letters[idx];
        for d in 0..10 {
            if used[d] {
                continue;
            }
            if d == 0 && leading_letters.contains(&current_letter) {
                continue; // no leading zero
            }

            used[d] = true;
            assignments.insert(current_letter, d as u8);
            if backtrack(idx + 1, letters, used, assignments, lhs, rhs, leading_letters) {
                return true;
            }
            used[d] = false;
            assignments.remove(&current_letter);
        }

        false
    }

    if backtrack(
        0,
        &letters,
        &mut used,
        &mut assignments,
        &lhs_words,
        rhs_word,
        &leading_letters,
    ) {
        Some(assignments)
    } else {
        None
    }
}

