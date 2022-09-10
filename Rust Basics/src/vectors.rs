//vectors are resizable arrays
pub fn run(){
    let mut array: Vec<i32>= vec![1,2,3,4,5,6];
    //full print
    println!("{:?}",array);

    //single value
    println!("one value {}",array[0]);

    //re-assign
    array[0]=60;
    println!("{:?}",array);

    //get length
    println!("Vector length {}",array.len());

    //get slice

    let slice = &array[0..2];
    println!("Vector Slice : {:?}", slice);
}