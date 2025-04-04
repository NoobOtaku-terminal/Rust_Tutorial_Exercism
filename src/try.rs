fn main() {
    let a: String = String::from("ルbcd");

    let chars: Vec<char> = a.chars().collect();
    let length = chars.len();
    
    // Now you can use a usize index to access each character.
    for i in 0..length {
        println!("{}", chars[i]);
    }
}
