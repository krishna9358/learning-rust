// struct 
struct Rectangle {
    width : f32,
    height : f32,
}

//implementing on struct same as class in other languages 
impl Rectangle{
    // member function 
    fn area(&self) -> f32{
        return self.width * self.height;
    }

    // static function
    fn square(size : f32) -> Rectangle{
        return Rectangle {
            width : size,
            height : size
        };
    }
}

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

    //mutability in string
    let mut mut_name = String::from("krishna ");
    mut_name.push_str("mohan");
    println!("{}", mut_name);

    //calling struct here
    let rectangle: Rectangle = Rectangle {
        width : 10.0,
        height : 10.0
    };
    println!("{} {} {}", rectangle.width, rectangle.height, rectangle.area());

    // calling static function
    let square = Rectangle::square(10.0);
    println!("{} {} {}", square.width, square.height, square.area());

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

// ownership rules
//  - each value in rust has an owner 
//  - there can only be one owner at a time 
//  - when the owner goes out of scope, the value will be dropped 

// borrowing rules 
//  - can have one mutable reference &mut 
//  - can have multiple immutable references 