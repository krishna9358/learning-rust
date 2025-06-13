
use std::fs::read_to_string;
use std::io::Error;

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

//enums are same as in typescripts
enum Direction{
    Up,
    Down,
    Left,
    Right
}

// enum with values
enum Shapes{
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32)
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

    //using enums
        let my_direction : Direction = Direction::Up;

    let my_shape : Shapes = Shapes::Circle(10.0);
    
    // pattern matching
    match my_direction{
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        // Direction::Right => println!("right"),
        _ => println!("default")    //default is used to handle the case when no other case is matched
    }  

    // try catch in rust
        let content : Result<String, Error> = read_to_string(  "file.txt");
    match content{
        Ok(content) => println!("{}", content),
        Err(error) => println!("Error: {}", error)
    }



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

fn calculate_area(shape : Shapes) -> f32{
    match shape {
        Shapes::Circle(radius) => return 3.14 * radius * radius,
        Shapes::Square(side) => return side * side,
        Shapes::Rectangle(width, height) => return width * height
    }
}


// option enum is used to handle the absence of a value
enum Option<T>{
    Some(T),
    None // None is a placeholder for the absence of a value
}

// code for option enum
fn option_enum(){
let x : Option<i32> = Some(5);
let y : Option<i32> = None;
println!("{:?}", x);
println!("{:?}", y);
}

// result enum
enum Result<T, E>{
    Ok(T),
    Err(E)
}

// code for result enum
fn divide(a : i32, b : i32) -> Result<i32, String>{
    if b == 0{
        return Err(String::from("Cannot divide by zero"));
    }
    return Ok(a / b);
}

