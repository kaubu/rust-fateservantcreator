use std::io::{Read, Write};
use std::fs::{self, OpenOptions};
use std::path::Path;
use glob::{glob_with, MatchOptions};

pub fn wo_to_file(object: String, path: String) { // Write Object to File
	let file = OpenOptions::new().write(true)
		.create_new(true)
		.open(path);

	let mut file = match file {
		Ok(file) => file,
		Err(error) => {
			panic!("Opening file failed: {:?}", error);
		}
	};

	file.write(object.as_bytes()).expect("Writing to file failed");
}

pub fn ro_from_file(file_path: String) -> String { // Read Object from file
	let mut file = fs::File::open(file_path).expect("Opening file failed");
	let mut contents = String::new();

	file.read_to_string(&mut contents).expect("Reading to string failed");
	contents
}

pub fn list_with_ext(folder: &str, extension: &str) {
	let new_path = format!("{}*.{}", folder, extension); // was ./{}/ but now {}. This means the folder input should be ./servants/
	let new_path: &str = &new_path[..];

	let options = MatchOptions {
		case_sensitive: false,
		require_literal_separator: false,
		require_literal_leading_dot: false,
	};

	for entry in glob_with(new_path, options).unwrap() {
		if let Ok(path) = entry {
			let string_path = path.display().to_string();
			let formatted_file = get_file_name(string_path); // get_file_name

			println!("{}", formatted_file);
		} else if let Err(error) = entry {
			panic!("Read files error: {:?}", error);
		}
	}
}

fn get_file_name(file: String) -> String { // File with Path. Path\\dir\\Test.svt
	let file = Path::new(&file).file_name().unwrap().to_str().unwrap(); // Only file name. Test.svt
	let file = file.to_string(); 

	let file_len = file.len();
	let file_name = file_len - crate::servant::SERVANT_EXTENSION_LEN - 1; // -1 accounts for the "."
	let file = &file[0..file_name];
	file.to_string() // Test
}