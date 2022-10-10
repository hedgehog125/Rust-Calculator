use std::io;
use std::io::{Error, ErrorKind};

fn main() {
	println!("Enter your first number...");
	let num1 = get_num_input();

	let operation = {
		println!("Now pick an operation with +, -, / or * ...");
		let mut operation = String::new();
		io::stdin().read_line(&mut operation);
		operation.chars().nth(0).unwrap()
	};

	println!("And your second number...");
	let num2 = get_num_input();

	let result: f32 = calc_result(num1, num2, operation).unwrap();
	println!("The result is {result}.");
}

fn get_num_input() -> f32 {
	let mut string_num = String::new();
	io::stdin().read_line(&mut string_num);

	string_num.trim().parse::<f32>().unwrap()
}

fn calc_result(num1: f32, num2: f32, operation: char) -> Result<f32, Error> {
	match operation {
		'+' => Ok(num1 + num2),
		'-' => Ok(num1 - num2),
		'/' => Ok(num1 / num2),
		'*' => Ok(num1 * num2),
		_ => Err(Error::new(ErrorKind::InvalidInput, "Invalid operation"))
	}
}