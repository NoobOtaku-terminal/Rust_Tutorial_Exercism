pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    let mut val =n*(n+1)/2;
    val*val
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    n*(n+1)*(2*n+1)/6
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    return square_of_sum(n)-sum_of_squares(n);
    
    
}
