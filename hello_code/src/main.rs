//INTEGER VARIABLE
fn main() {
    let num:u8=5;

    println!("the num is {}", num);
}

//Note: By  default declared  variables are immutable. we can not change the values after declaration. 
// Use "mut" to make mutable varibale

/*fn main() {
    let num:u8=5; //immutable variable.
    let mut nums:u8 = 24; //mutable variable.

    println!("the num is {}", num);
}*/

/*string has 2 type 
    1) &str -  fixed length string literal. Stored in read only data memory(rodata)
    2) String - Dynamic length string literal. stored in Heap memory.
*/

// 1)  &str STRING LITERAL
/*fn main() {
    let string_var:&str = "Hi, this is singhal";

    println!("The sentense is: {}", string_var);
}*/

// 2) String 
/* fn main() {
    let mut string_var= String:: from("Hi, this is krishan singhal. ");
    string_var.push_str("A blockchain developer.");

    println!("The sentense is: {}", string_var);
}*/

//TUPPLE
/*fn main() {
    
    let emp_info:(&str,u8) = ("Ramesh",50);

    //destructuring
    let (employee_name, employee_age) = emp_info;

    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    // print!("employee name = {}, employee age = {}", emp_name, emp_age);
    print!("employee name = {}, employee age = {}", employee_name, employee_age);
}*/



//FUNCTIONS
/*fn main() {
    // test(24);
    let num1:u8 = 10;
    let num2:u8 = 20;
    let result:u8 = add(num1, num2);
    println!("the sum of 2 numbers is {}", result)
}
//fn test(age:u8){
//     println!("Hello, Krishan, your age is {}",age);
// }


fn add(var1:u8, var2:u8)->u8{
    return var1+var2;
}*/

// OWNERSHIP:every value should have an owner.
// every value should have only one owner.
// memory will be free after variable goes out of the scope. 

//This function is working in stack memory and everything is fixed in it so we are not getting an error.
/*fn main(){
    let a =5;
    let b = a;

    println!("a=  {}",a);
    println!("b= {}", b);
}*/

//lets see the ownership exaple:
//here ownership of string is transferring from str1 to str2, we are doing str2= str1.
/*fn main() {
    let str1 = String::from ("hi, krishan");
    let str2 = str1;

    // println!("str1 = {}", str1); //if we'll print this we'll get error because str1 is valid now.
    println!("str2 = {}", str2);
}*/

/*fn main(){
    let x:String = String::from("hi billu");
    process_item(x);
    // println!("the value is {}", x);
}
fn process_item(item:String){
    println!("the new value is {}",item);
}*/

/*fn main(){
    let s1:String = get_string(); //s1 is owner of hello.
    println!("This is s1:{}",s1);

    let s2:String= String::from("world"); //s2 owner of world.
    
    let s3:String = send_get_string(s2); //transferring the ownership of world to s3.

    // println!("s2:{}", s2);
    println!("This is s3:{}",s3);
}

fn get_string()->String{
    let new_string =String::from("hello");
    return new_string;
}

fn send_get_string(received_string:String)->String {
    return received_string;
}*/

// Ownership solution
//1) USING TUPPLE

/*fn main(){
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);

    println!("The length of {} is {}",s2,len);
}

fn calculate_length(s:String)->(String,usize){
    let length:usize = s.len();
    return (s,length);
}*/

//2) USING clone METHOD
/*fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(s1.clone());

    println!("The length of {} is {}",s1,len);
}

fn calculate_length(s:String)->usize{
    let length:usize = s.len();
    return length;
}*/

// 3) THIRD METHOD (BORROWING)
/*fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //Passing the reference of s1

    println!("The length of {} is {}",s1,len);
}

fn calculate_length(s2:&String)->usize{
    return s2.len();
}*/


// MUTABLE STRING USING REFERENCE

/*fn main(){
    let mut str1:String = String::from("Hello ");
    append_string(&mut str1);
    println!("The new string is : {}", str1);
}

fn append_string(str2: &mut String){
    str2.push_str("World");
}*/

