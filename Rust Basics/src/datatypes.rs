pub fn run(){

    let x=9;
    let y=9.7;
    let c="char";

    //find max size
    println!("max i:32 - {}",std::i32::MAX);
    println!("max i:32 - {}",std::i64::MAX);

    //boolean
    let u=true;
    println!("U={}",u);

    //boolean from expression
    let p= 9>10;
    println!("p is {}",p);

    //char
    let o='p';
    let i='\u{1F636}';

    println!("{:?}",(o,i));
}