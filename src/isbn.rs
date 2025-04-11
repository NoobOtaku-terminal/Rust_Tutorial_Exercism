/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // todo!("Is {isbn:?} a valid ISBN number?");
    let mut i=0;
    let mut count =0;
    let mut value =0;
    for ch in isbn.chars(){
        if ch.is_digit(10){
            value += ch.to_digit(10).unwrap()*(10-i);
            i+=1;
        }else if ch =='X' && i==9{
            value += 10;
            i+=1;
        }else if ch != '-'{
            return false;
        }
        
    }
    i==10 && value%11 == 0
}
