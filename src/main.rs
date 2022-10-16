

#![allow(non_snake_case)]
use std::option::Option;
fn main() {

    let  i = 3;
    let  five = Some(i);
    let six = plus_one(five);
    let mut Z:i32 = 0;

    match six {
        Some(x) => {println!("Result {}", x);
                          Z = x + 5;
                        },
        None => println!("The value was null"),
    }
   
    println!("I can use the value from the match out here {}",Z);


}

fn plus_one(x:Option<i32>) -> Option<i32>{
    
    match x{
        None => None,
        Some(i) => Some(i+1),
    } 
    

        
    }


