//reference pointers

pub fn run(){

    //primitive array
    let arr1=[1,2,3,4];
    let arr2=arr1;
    println!("Values: {:?}",(arr1,arr2));

    //vector
    let vec1=vec![1,2,3];
    let vec2=&vec1;
    println!("Vector values {:?}",(&vec1,vec2));



}