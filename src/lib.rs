use std::io::{self, Write};
use std::fs;

mod servant;
mod servant_fs;

pub fn start() {
	// Check if servants folder exists, and if it doesn't, create it.
	fs::create_dir_all(servant::SERVANT_PATH).expect("Error creating servants folder");

	loop {
		println!("[ Options ]
=[1][ Create a new Servant ]
=[2][ Display an existing Servant ]
=[3][ Delete an existing Servant ]
=[4][ List all existing Servants ]
=[5][ Exit the program ]");

		let option: i32 = input(">> ").parse().unwrap_or(-1);

		match option {
			-1 => println!("Not a number!"), // Couldn't parse input into i32
			1 => create_servant(), // Create a servant
			2 => display_servant(), // Display a servant
			3 => delete_servant(), // Delete a servant
			4 => list_servants(), // List all servants
			5 => break, // Exit
			_ => println!("Not an option!") // Anything other option
		}
	}
}

fn input(message: &str) -> String { // Gets input
	let mut input = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("Failed to read line");

	input.trim().to_owned()
}

fn create_servant() {
	let stars_input: u32 = input("Stars: ").parse().expect("Stars are usually from 1 to 5");

	let servant_obj = servant::Servant {
		stars: stars_input,
		class: input("Class: "),
		true_name: input("True Name: "),
		alignment: input("Alignment: "),
		attributes: input("Attributes: "),
		qualified_servant_classes: input("Qualified Servant Classes: "),
		strength: input("Strength: "),
		endurance: input("Endurance: "),
		agility: input("Agility: "),
		mana: input("Mana: "),
		luck: input("Luck: "),
		np: input("NP: "),
		class_skills: input("Class Skills: "),
		personal_skills: input("Personal Skills: ")
	};

	servant_obj.save();
}

fn display_servant() {
	let file_name = input("Servant to load: ");
	let servant_obj: servant::Servant = servant::load(file_name);

	servant_obj.display();
}

fn delete_servant() {
	let file_name = input("Servant to delete: ");
	let path = format!("{}{}.{}", servant::SERVANT_PATH, file_name, servant::SERVANT_EXTENSION);

	match servant::delete(path) {
		Err(error) => panic!("Error deleting servant: {:?}", error),
		_ => println!("Servant deleted successfully.")
	}
}

fn list_servants() {
	servant_fs::list_with_ext(servant::SERVANT_PATH, servant::SERVANT_EXTENSION);
}