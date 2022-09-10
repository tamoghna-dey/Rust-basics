//structs - used to create custom datatype

//traditional struct
struct Color {
    red:u8,
    green:u8,
    blue:u8
}

//tuple struct
struct Nice(u8,u8,u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    //Construct person

    fn new(first: &str, last:  &str)->Person{
        Person { first_name: first.to_string(),last_name: last.to_string()}
    }
}
pub fn run(){
    let c= Color{
        red:225,
        green: 0,
        blue:90
    };

    println!("Color : {} {} {}",c.red,c.blue,c.green);

    let k=Color{
        red:205,
        green: 10,
        blue:91 
    };

    println!("Color : {} {} {}",k.red,k.blue,k.green);

    let o=Nice(233,45,67);

    println!("Color: {} {} {}",o.1,o.2,o.0);

    let mut l=Person::new("John","Dohl");
    


}