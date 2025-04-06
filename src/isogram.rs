use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    // todo!("Is {candidate} an isogram?");
    let candidates = candidate.to_lowercase();
    let mut set = HashSet::new();
    for ch in candidates.chars(){
        if set.contains(&ch){
            return false;
        }
        if ch.is_alphabetic(){
            set.insert(ch);
        }
}
    return true;
}
