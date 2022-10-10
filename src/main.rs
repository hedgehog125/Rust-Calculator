use std::io;
//use std::io::{Error, ErrorKind};

fn main() {
	println!("Enter your first number...");
	let num1 = get_num_input();

	println!("Now pick an operation with +, -, / or * ...");
	let operation = get_operation();

	println!("And your second number...");
	let num2 = get_num_input();

	let result: f32 = calc_result(num1, num2, operation);
	println!("The result is {result}.");
}

fn get_num_input() -> f32 {
	loop {
		let mut string_num = String::new();
		match io::stdin().read_line(&mut string_num) {
			Ok(_) => {
				match string_num.trim().parse::<f32>() {
					Ok(value) => return value,
					Err(_) => {}
				}
			},
			Err(_) => {}
		}
		println!("Invalid number. Try again...");
	}
}

fn get_operation() -> Operation {
	loop {
		let mut operation = String::new();
		match io::stdin().read_line(&mut operation) {
			Ok(_) => {
				match operation.chars().nth(0) {
					Some(value) => {
						match value {
							'+' => return Operation::Add,
							'-' => return Operation::Subtract,
							'/' => return Operation::Divide,
							'*' => return Operation::Multiply,
							_ => {}
						}
					},
					None => {}
				}
			},
			Err(_) => {}
		}

		println!("Invalid operation. Try again...");
	}
}

enum Operation {
	Add,
	Subtract,
	Divide,
	Multiply
}

fn calc_result(num1: f32, num2: f32, operation: Operation) -> f32 {
	match operation {
		Operation::Add => num1 + num2,
		Operation::Subtract => num1 - num2,
		Operation::Divide => num1 / num2,
		Operation::Multiply => num1 * num2
	}
}