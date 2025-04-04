use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = sorted_lowercase(word);
    let mut set = HashSet::new();

    for &st in possible_anagrams {
        let lower_st = st.to_lowercase();

        if lower_st == word.to_lowercase() {
            continue; // Skip identical words
        }

        if sorted_lowercase(st) == sorted_word {
            set.insert(st);
        }
    }

    set
}

// Helper function: Convert word to lowercase and return sorted characters as String
fn sorted_lowercase(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable(); // Faster sorting since we don't need stability
    chars.into_iter().collect()
}





