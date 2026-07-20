use std::io::{ Write};
use std::io;

fn run_calculator() {
    println!("----Rust Calculator----");
loop {
   let exit = menu();
   if exit {
	break;
  }
 }
}
fn menu() -> bool {
   println!("Choose the Function");
 io::stdout().flush().unwrap(); 
   println!("1.Addition");
   println!("2.substraction");
   println!("3.Multiplication");
   println!("4.Division");
   println!("5. Exit");
   println!("Choose an option");

let mut choice = String::new();
 io::stdin().read_line(&mut choice).unwrap();
   match choice.trim() {
    "1" => {
	add();
	false
   }
    "2" => {
	substract();
	false
   }
    "3" => {
	multiply();
	false
   }
    "4" => {
	divide();
	false
   }
    "5" => {
   println!("Goodbye user");
	true
   }
    _=> {
   println!("invalid input");
	false   
   }
  }
 }

//1st & 2nd number function
fn numbers() -> (f32, f32) {
 let mut input = String::new();
 println!("Enter the First Number");
 io::stdin().read_line(&mut input).unwrap();
let a: f32 = input.trim().parse().unwrap();

input.clear();
println!("Enter the second Number");
 io::stdin().read_line(&mut input).unwrap();
let b: f32 = input.trim().parse().unwrap();

 (a, b)
}
/*for learning purposes i nver refactored
the code ,, you might notice the code
repetion in below arithmetic functions*/
fn add() {
 let (a, b) =  numbers();
 let results = a + b;
 println!("The total sum = {}", results);
}
fn substract() {
 let (a, b) =  numbers();
 let results = a - b;
 println!("The difference = {}", results);
}
fn multiply() {
 let (a, b) =  numbers();
 let results = a * b;
 println!("The product = {}", results);
}
fn divide() {
 let (a, b) =  numbers();
 let results = a / b;
println!("The quotient = {}", results);
}
fn main() {

   run_calculator();

}
