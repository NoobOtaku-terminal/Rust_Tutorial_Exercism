fn main(){
    let rohan:String = String::from("Rohanな");
    for ch in rohan.chars(){
        println!("{}",ch);
    }
    println!("{:?}",rohan);
}