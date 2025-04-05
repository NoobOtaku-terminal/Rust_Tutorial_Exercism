pub fn is_prime(n:u32) ->bool{
    for i in 2..n{
        if n%i==0{
            return false;
        }
    }
    return true;
}
pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut i =2;
    let mut count = 0;
    let mut ans=2;
    while count <= n+1{
        if is_prime(i){
            count+=1;
        }
 
        if count == n+1{
            ans = i;
            break;
        }
               i+=1;
       
    }
    ans
    
}
