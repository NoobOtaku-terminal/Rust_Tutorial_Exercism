pub fn abbreviate(phrase: &str) -> String {
    let mut ans = String::new();
    let mut new_word = true;
    let chars: Vec<char> = phrase.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if c.is_ascii_alphabetic() {
            if new_word {
                ans.push(c.to_ascii_uppercase());
                new_word = false;
            } else if c.is_ascii_uppercase() {
                // camelCase check: only include if preceded by lowercase
                if i > 0 && chars[i - 1].is_ascii_lowercase() {
                    ans.push(c);
                }
            }
        } else if c != '\'' {
            // Treat punctuation (except apostrophes) as word boundaries
            new_word = true;
        }
    }

    ans
}

