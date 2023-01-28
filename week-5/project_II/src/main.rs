// Rust program to determine age and experience pass

use std::io;

fn main() {

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();


	println!("Enter name");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Enter your age: ");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let age:i32 = input2.trim().parse().expect("Not a valid number");

	println!("Are you experienced yes/no");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let exp:bool = input3.trim().parse().expect("Not a valid number");

	if "age" >= 40 && exp:bool == bool{
	println!("Hello {} your incentive is 1560000 ", input1);
	

	 "else" "age" >= 30 && < 40 && exp:bool == bool{
	println!("Hello {} your incentive is 1480000 ", input1);
    } } else {
    	println!("Your not hired {}",input1 );
    }

    "else" "age" < 28 && exp:bool == bool{
    	println!("Hello {} your incentive is 1300000 ", input1);
    }
    else {
    	println!("hello {} Your incentive is 1000000",input1);
    }
}
