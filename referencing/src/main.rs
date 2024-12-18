fn main(){
    let x:u8=10;
    let y= &x; //Y is reference of value of x;
    println!("the value of x is {}", y);
}

//Auto Dereferencing
/*fn main(){
    let mut x = 5;
    x=x+1;//6
    let y = &mut x; 
    *y=*y+1;//7 //auto dereferencing
    println!("value of x= {}", y);
}*/

//dangling referencing
/*fn main(){
    reference_to_nothing= create_string_ref();  //this is not referencing anythig.
                                    //The reference of s will drop after create_string_ref function execution.
}

fn create_string_ref()->&String {
    let s:String= String::from("hello");
    return &s;
}*/

//ARRAY

/*fn main(){
    // let mut arr1:[u8;5]; //array declaration
    let mut arr1;
    arr1  =[1,2,3,4,5];

    println!("arra1[0]= {}", arr1[0]);
    println!("arr1={:?}",arr1);

    arr1[2]=30;
    println!("arr1={:?}",arr1);

    println!("Array length is {}",arr1.len());
}*/

//ARRAY Referencing
/*fn main(){
    let arr:[&str;3]=["Hello", "world", "Coder"];
    write_arr(&mut arr); //reference of array
    println!("arr={:?}",arr);
}

//will change in existing array
fn write_arr(arr2:&mut [&str; 3]){
    arr2[0]="Fellow";
    println!("arr2={:?}",arr2);
}

//will create a new array
// fn main(mut arr1:[&str;3]){
//     arr1[0]="Fellow";
//     println!("arr1={:?}",arr1);
// }*/


//VECTOR- Dynamic array
/*fn main(){

    let mut v:Vec<i32> = Vec::new(); //declaration
    // let mut v = Vec::<i32>::new(); //second way of declaration

    v.push(1);
    v.push(2);
    v.push(3);

    // let v = vec![1,2,3,4,5]; // initialized vector

    println!("Vector V= {:?}", v);
}*/

//function in vector
/*fn main(){
    let mut vrr: Vec<&str> = vec!["hello", "world", "coders"];
    write_vrr(&mut vrr);
    println!("vrr={:?}",vrr);
}

fn write_vrr(vrr2: &mut Vec<&str>){
    vrr2.push("Eater");
    println!("vrr2={:?}",vrr2);
}*/

//SHADOWING:: can create multiple variable of same name and different data type.
/*fn main(){
    let x = 5; //integer
    println!("x={}",x);
    let x = "Hello"; //string
    println!("x={}",x);
    let x = x.len(); //integer
    println!("x={}",x);
}*/

//FOR LOOP

/*fn main(){
    let arr = [1,2,3];

    for element in &arr{
        println!("{}",element);
    }
}*/

//MATCH: Similar to switch

/*fn main(){
    let  num = 5;

    match num {
        1 | 3 => println!("number is one or three"),
        2 | 4 => println!("number is two or four"),
        5 => println!("number is five"),
        _=>println!("invalid number")
    }
}*/

//USER INPUTS
/*use std::io;
fn main(){
    let mut input: String = String::new();
    println!("Please enter your name");
    io::stdin()
    .read_line(&mut input)
    .expect("input failed");
    println!("user input:{}",input);
}*/
