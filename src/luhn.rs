/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let mut i =0;
    let mut sum=0;
    if code.len() <=1 {
        return false;
    }
    for ch in code.chars().rev(){
        if ch != ' ' && (ch <'0' || ch >'9'){
            return false;
        }
        if ch ==' '{
            continue;
        }
        if i%2 == 0{
            sum+= ch as u32 - '0' as u32;
        }else{
            let mut val = ch as u32 - '0' as u32;
            val = val*2;
            if val > 9 {
                val -=9;
            }
            sum += val;
        }
        i+=1;
    }
    if i <=1{
        return false;
    }
     sum%10 ==0
    
}
