fn main() {
    // print
    println!("Hello, world!");
    // function calling
    println!("{}",is_even(2));
    // string 
    let name : String = String::from("krishna");
    let last_name  = "mohan";
    println!("{} {}",name,last_name);

    // vectors
    let vector : Vec<i32> = vec![1,2,3,4];
    println!("{:?}", vector );

    //for loops
    for i in 0..100 {
        println!("{}", i);
    }

    // while loops
    let mut x = 0;
    while x < 100 {
        println!("{}", x);
        x += 1;
    }

    let x = 2;  // immutable variable
    let mut y = 2; // mutable variable
    
    println!("{} {}", x, y);  
    y = 3;    
    println!("{} {}", x, y);  

}

// fucntion initialization 
fn is_even(n : u32) -> bool {
    //conditional statement
    if  n % 2 == 0 {
        return true;
    } else{
        return false;
    }
}