use std::io::{self, Read, Write};
use std::fs::{self, OpenOptions};
use std::path::Path;
use serde::{Serialize, Deserialize};
use glob::{glob_with, MatchOptions};

const MENU_STRING: &str = "[ Options ]
=[1][ Create a new Servant ]
=[2][ Load an existing Servant ]
=[3][ Delete an existing Servant ]
=[4][ List all existing Servants ]
=[5][ Exit the program ]";
const SERVANT_FOLDER: &str = "servants";
const SERVANT_EXTENSION: &str = "svt";
const SERVANT_EXTENSION_LEN: usize = SERVANT_EXTENSION.len();
const STAR_SOLID: &str = "★";

#[derive(Serialize, Deserialize, Debug)]
struct Servant {
	stars: u32,
	class: String,
	true_name: String,
	alignment: String,
	attributes: String,
	qualified_servant_classes: String,
	strength: String,
	endurance: String,
	agility: String,
	mana: String,
	luck: String,
	np: String,
	class_skills: String,
	personal_skills: String
}

pub fn input(message: &str) -> String { // Gets input
	let mut input = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("Failed to read line");

	input.trim().to_owned()
}

pub fn clear() { // Clears the screen
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn init_menu() {
	// Check if servants folder exists, and if it doesn't, create it.
	fs::create_dir_all(SERVANT_FOLDER).expect("Error creating servants folder");

	loop {
		// input("Press ENTER to continue...");
		// clear();
		println!("{}", MENU_STRING);
		let option = input(">> ");
		let option: i32 = option.parse().unwrap_or(-1);

		match option {
			-1 => {
				println!("Not a number!");
			},
			1 => { // Create a new servant
				create_servant();
			},
			2 => { // Load an existing servant
				let servant: Servant = load_servant(input("Servant file name: "));
				print_servant_stats(servant);
			},
			3 => { // Delete an existing servant
				let file_name = input("Servant file name: ");
				let file_path = format!("./{}/{}.{}", SERVANT_FOLDER, file_name, SERVANT_EXTENSION);

				match delete_servant(file_path) {
					Err(error) => {
						panic!("Error deleting servant: {:?}", error);
					},
					_ => println!("Servant deleted successfully.")
				}
			},
			4 => { // List all existing servants
				get_servants();
			},
			5 => { // Exit the program
				break;
			},
			_ => {
				println!("Not an option!");
			}
		}
	}
}

fn create_servant() {
	let stars_input: u32 = input("Stars: ").parse().expect("Stars are usually from 1 to 5");

	let servant = Servant {
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

	let servant_serialized = serde_json::to_string(&servant).expect("Serializing servant failed");
	let servant_path = format!("./{}/{}_{}.{}", SERVANT_FOLDER, servant.class, servant.true_name, SERVANT_EXTENSION);

	wo_to_file(servant_serialized, servant_path);
	println!("Servant successfully created.");
}

fn wo_to_file(serial_obj: String, path: String) { // Write Object to File
	let file = OpenOptions::new().write(true)
		.create_new(true)
		.open(path);

	let mut file = match file {
		Ok(file) => file,
		Err(error) => {
			panic!("Opening file failed: {:?}", error);
		}
	};

	file.write(serial_obj.as_bytes()).expect("Writing to file failed");
}

fn r_from_file(file_path: String) -> String { // Read from file
	let mut file = fs::File::open(file_path).expect("Opening file failed");
	let mut contents = String::new();

	file.read_to_string(&mut contents).expect("Reading to string failed");
	contents
}

fn load_servant(file_name: String) -> Servant {
	let path = format!("./{}/{}.{}", SERVANT_FOLDER, file_name, SERVANT_EXTENSION);
	let servant = r_from_file(path).to_string();
	let servant: Servant = serde_json::from_str(&servant).expect("Unserializing servant failed");

	servant
}

fn delete_servant(file_path: String) -> io::Result<()> {
	fs::remove_file(file_path).unwrap();
	Ok(())
}

fn get_servants() { // List all .svt files.
	list_files_in_local_dir_with_ext(SERVANT_FOLDER, SERVANT_EXTENSION);
}

fn list_files_in_local_dir_with_ext(folder: &str, extension: &str) {
	let new_path = format!("./{}/*.{}", folder, extension);
	let new_path: &str = &new_path[..];

	let options = MatchOptions {
		case_sensitive: false,
		require_literal_separator: false,
		require_literal_leading_dot: false,
	};

	for entry in glob_with(new_path, options).unwrap() {
		if let Ok(path) = entry {
			let string_path = path.display().to_string();
			let formatted_file = print_formatted_file(string_path);

			println!("{}", formatted_file);
		} else if let Err(error) = entry {
			panic!("Read files error: {:?}", error);
		}
	}
}

fn print_formatted_file(file: String) -> String {
	let file = Path::new(&file).file_name().unwrap().to_str().unwrap(); // File with Path. Path\\dir\\Test.svt
	let file = file.to_string(); // Test.svt

	let file_len = file.len();
	let file_name = file_len - SERVANT_EXTENSION_LEN - 1;
	let file = &file[0..file_name];
	file.to_string() // Test
}

fn print_servant_stats(servant: Servant) {
	let stars_display = get_stars(servant.stars);

	println!("[Servant Stats for {class} {stars_num}{star}]
Class: {class}
True Name: {true_name}
Stars: {stars}
Alignment: {alignment}
Attributes: {attributes}
Qualified Servant Classes: {qualified_servant_classes}

Strength: {strength}
Endurance: {endurance}
Agility: {agility}
Mana: {mana}
Luck: {luck}
NP: {np}

Class Skills: {class_skills}
Personal Skills: {personal_skills}",
	stars_num=servant.stars,
	star=STAR_SOLID,
	class=servant.class,
	true_name=servant.true_name,
	stars=stars_display,
	alignment=servant.alignment,
	attributes=servant.attributes,
	qualified_servant_classes=servant.qualified_servant_classes,
	strength=servant.strength,
	endurance=servant.endurance,
	agility=servant.agility,
	mana=servant.mana,
	luck=servant.luck,
	np=servant.np,
	class_skills=servant.class_skills,
	personal_skills=servant.personal_skills);
}

fn get_stars(stars: u32) -> String {
	let usize_stars: usize = stars as usize;
	STAR_SOLID.to_string().repeat(usize_stars)
}