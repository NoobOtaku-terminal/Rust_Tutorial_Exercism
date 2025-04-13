#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    if first_list == second_list {
        Equal
    } else if is_sublist(first_list, second_list) {
        Sublist
    } else if is_sublist(second_list, first_list) {
        Superlist
    } else {
        Unequal
    }
}

fn is_sublist(short: &[i32], long: &[i32]) -> bool {
    if short.is_empty() {
        return true;
    }

    long.windows(short.len()).any(|window| window == short)
}
