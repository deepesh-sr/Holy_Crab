use core::num;
use std::io::{self, Write, stdin, stdout}; 

fn main(){
    println!("You Rust Calculator"); 
    loop {

        let a = read_number("first number : "); 
        let b = read_number("second number : "); 
        let op = read_operator(); 

        if op == 'q' {
            break;
        }
        
        match calculate(a, b, op){
            Ok(number)=> println!("{a}  {op} {b} =  {number}"), 
            Err(_)=> println!("Out of the scope of the project")
        }
        
    }
}

fn read_number(prompt : &str)->f64{

    loop {
        let mut input = String::new(); 
        
        print!("{}", prompt); 
        stdout().flush().unwrap(); 
        
        match stdin().read_line(&mut input) {
            Ok(_)=> {
                match input.trim().parse::<f64>() {
                    Ok(number)=> return number ,
                    Err(_)=> print!("Not a valid number")
                }
            },
            Err(_)=> println!("Give a valid input, i am having error in taking input, lol")
        }    
    }
}

fn read_operator()-> char {

    loop {
        let mut input = String::new();
        
        print!("enter your operator : "); 
        stdout().flush().unwrap(); 

        match stdin().read_line(&mut input) {
            Ok(_)=> {


                let trimmed = input.trim(); 

                if trimmed == "q" {
                    return 'q'
                }

                let mut chars = trimmed.chars(); 

                match (chars.next() ,chars.next()) {
                    (Some(op @ ('+'| '-' | '*' | '/' )), None)=> return op,
                    _=> println!("Not a valid operator ")
                }
            },
            Err(_)=> println!("Give a valid input, i am having error in taking input, lol")
        }    
    }
}


fn calculate( a : f64 , b : f64 , op : char)-> Result<f64, &'static str> {

    match op {
        '+' => Ok(a + b), 
        '-' => Ok(a - b),
        '*' => Ok(a * b), 
        '/' if b != 0.0 => Ok(a/b),
        '/' => Err("Number not divisible by zero"), 
        _ => Err("Out of scope of our calculator")
    }

}