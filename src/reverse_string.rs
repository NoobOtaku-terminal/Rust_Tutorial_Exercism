pub fn reverse(input: &str) -> String {
    let temp: Vec<char> = input.chars().collect();
    let mut ans = String::new();

    let mut i = temp.len() as isize - 1; // Convert to isize
    while i >= 0 {
        ans.push(temp[i as usize]); // Convert back to usize for indexing
        i -= 1;
    }

    ans
}
// pub fn reverse(input: &str) -> String {
//     let mut ans = String::new(); // Make ans mutable
//     for ch in input.chars().rev() { // Efficiently iterate in reverse
//         ans.push(ch);
//     }
//     ans // No need for explicit return
// }
