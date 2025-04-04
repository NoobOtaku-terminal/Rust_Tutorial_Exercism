#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
   let a_len = first_list.len();
    let b_len = second_list.len();
    
    if a_len == b_len && a_len==0{
        Comparison::Equal
    }else if a_len > b_len{
        is_super(first_list,second_list)
    }else if a_len < b_len{
        is_sub(first_list,second_list)
    }else {
        is_equal(first_list,second_list)
    }
}
fn is_super(a:&[i32] , b:&[i32]) ->Comparison{
    // since a's lenght is more than b's length, either a is a super list of b or they are unequal
    // we try to find b in a
    for i in 0..=a.len()-b.len(){
        let mut count:i32 = 0;
        for j in 0..b.len(){
            if b[j] == a[i+j]{
                count +=1;
            }else{
                break;
            }
        }
        if count == b.len() as i32 {
           return Comparison::Superlist;
        }
    }
  return  Comparison::Unequal;

}
fn is_sub(a:&[i32] , b:&[i32])->Comparison{
    // since b's length is  more than a's lenght either a is a sublist or either not 
    // we try to find a in b
    for i in 0..=b.len()-a.len(){
        let mut count:i32=0;
        for j in 0..a.len(){
            if b[i+j]==a[j]{
                count +=1;
            }else{
                break;
            }
        }
        if count == a.len() as i32 {
           return Comparison::Sublist;
        }
    }
  return  Comparison::Unequal;
}
fn is_equal(a:&[i32] , b:&[i32])->Comparison{
    // since same lenght either equal or not
    for i in 0..a.len(){
        if a[i] != b[i]{
          return  Comparison::Unequal;
        }
    }
   return Comparison::Equal;
}


#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
//     if first_list == second_list {
//         return Comparison::Equal;
//     }
//     if is_sublist(first_list, second_list) {
//         return Comparison::Sublist;
//     }
//     if is_sublist(second_list, first_list) {
//         return Comparison::Superlist;
//     }
//     Comparison::Unequal
// }

// fn is_sublist(smaller: &[i32], larger: &[i32]) -> bool {
//     if smaller.is_empty() {
//         return true; // An empty list is a sublist of any list
//     }
//     larger.windows(smaller.len()).any(|window| window == smaller)
// }


// #[derive(Debug, PartialEq, Eq)]
// pub enum Comparison {
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

// pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
//     let a_len = first_list.len();
//     let b_len = second_list.len();
    
//     if a_len == 0 && b_len == 0 {
//         return Comparison::Equal;
//     } else if a_len == b_len {
//         if first_list == second_list {
//             return Comparison::Equal;
//         }
//         return Comparison::Unequal;
//     } else if a_len > b_len {
//         if kmp_search(first_list, second_list) {
//             return Comparison::Superlist;
//         }
//     } else {
//         if kmp_search(second_list, first_list) {
//             return Comparison::Sublist;
//         }
//     }
//     Comparison::Unequal
// }

// fn kmp_search(text: &[i32], pattern: &[i32]) -> bool {
//     let n = text.len();
//     let m = pattern.len();
//     if m == 0 {
//         return true;
//     }
    
//     let lps = compute_lps(pattern);
//     let mut i = 0; // Pointer for text
//     let mut j = 0; // Pointer for pattern

//     while i < n {
//         if text[i] == pattern[j] {
//             i += 1;
//             j += 1;
//             if j == m {
//                 return true; // Found pattern
//             }
//         } else if j > 0 {
//             j = lps[j - 1]; // Use LPS to skip comparisons
//         } else {
//             i += 1;
//         }
//     }
//     false
// }

// fn compute_lps(pattern: &[i32]) -> Vec<usize> {
//     let m = pattern.len();
//     let mut lps = vec![0; m];
//     let mut len = 0;
//     let mut i = 1;

//     while i < m {
//         if pattern[i] == pattern[len] {
//             len += 1;
//             lps[i] = len;
//             i += 1;
//         } else if len > 0 {
//             len = lps[len - 1];
//         } else {
//             lps[i] = 0;
//             i += 1;
//         }
//     }
//     lps
// }
