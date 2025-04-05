// use num_traits::pow;
pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
       1u64<<(s-1)

}

pub fn total() -> u64 {
    // todo!();
    // let val = 2.pow(65);
    let mut sum:u64 =0;
    for i in 1..65{
        sum += square(i);
    }
    sum
    
}
