pub fn run(){
    let mut count=0;

    
    /*loop{
        count+=1;
        println!("Number {}",count);

        if count ==20{
            break;
        }
    }*/

    //while loop (fizz buzz, if number div by 3 print fizz, if div by 5 print buzz, if by both print fizz buzz)

    //let mut no=12;
    println!("WHILE LOOP FIZZ BUZZ");
    while count<=100{
        if count%15==0{
            println!(" {}- Fizz buzz",count);
        }
        else if count %5==0{
            println!("{}- Buzz",count);
        }
        else if count%3==0 {
            println!("{}- Fizz",count);
        }
        else{
            println!("{}- useless ",count);
        }

        count+=1;
    }

    println!("FOR LOOP FIZZ BUZZ");

    for x in 0..100{
        if x%15==0{
            println!(" {}- Fizz buzz",x);
        }
        else if x %5==0{
            println!("{}- Buzz",x);
        }
        else if x%3==0 {
            println!("{}- Fizz",x);
        }
        else{
            println!("{}- useless ",x);
        }

    }


}