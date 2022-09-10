pub fn run(){

    //static string
    let hello="hello world";
    println!("{}",hello);

    //growable, heap allocated string
    let mut nice=String::from("Nice ");

    //to push one character
    nice.push('!');
    println!("{}", nice);

    //to push a string
    nice.push_str(" How are you?");
    println!("{}", nice);

    //capacity
    println!("Capacity: {}",nice.capacity());

    //contains
    println!("Contains : {}", nice.contains("How"));

    //replace
    println!("Replace: {}", nice.replace("How", "What"));

    //string with capacity
    let mut y=String::with_capacity(10);
    y.push('y');
    y.push('o');

    println!("String y is {}", y);

    //assertion-to check if left is equal right
    assert_eq!(10,y.len());
    assert_eq!(10,y.capacity());
    
}