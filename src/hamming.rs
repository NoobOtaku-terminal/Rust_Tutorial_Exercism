/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // todo!("What is the Hamming Distance between {s1} and {s2}");
    if s1.len() != s2.len(){
        return None;
    }
    let str1 = s1.as_bytes();
    let str2 = s2.as_bytes();
    let mut count = 0;
    for i in 0..str1.len(){
        if (str1[i] as u8)^(str2[i] as u8) != 0{
            count += 1;
        } 
    }
    return Some(count as usize);
    
}
