pub fn is_armstrong_number(num: u32) -> bool {
    let mut x = num;
    let mut  count =0;
    while x > 0{
        count += 1;
        x = x/10;
    }
    x = num;
    let mut sum:u32 =0;
    while x > 0{
        let val = x%10;
        x = x/10;
        sum += u32::pow(val,count);
    }
    sum==num
    
}
