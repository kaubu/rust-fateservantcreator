use std::io;
use std::fs;
use serde::{Serialize, Deserialize};

pub const SERVANT_PATH: &str = "./servants/";
pub const SERVANT_EXTENSION: &str = "svt";
pub const SERVANT_EXTENSION_LEN: usize = SERVANT_EXTENSION.len();
pub const STAR_SOLID: &str = "★";

#[derive(Serialize, Deserialize, Debug)]
pub struct Servant {
	pub stars: u32,
	pub class: String,
	pub true_name: String,
	pub alignment: String,
	pub attributes: String,
	pub qualified_servant_classes: String,
	pub strength: String,
	pub endurance: String,
	pub agility: String,
	pub mana: String,
	pub luck: String,
	pub np: String,
	pub class_skills: String,
	pub personal_skills: String
}

impl Servant {
	pub fn save(&self) {
		let servant = serde_json::to_string(self).expect("Serializing servant failed");
		let servant_path = format!("{}{}_{}.{}", SERVANT_PATH, self.class, self.true_name, SERVANT_EXTENSION);
		// ^^^ was ./{}/ but now {}. This means the folder input should be ./servants/

		crate::servant_fs::wo_to_file(servant, servant_path);
		println!("Servant successfully created.");
	}

	pub fn display(&self) {
		let stars_display = get_stars(self.stars);

		println!("== [ Servant Stats for {class} {stars_num}{star} ]
=== Class: {class}
=== True Name: {true_name}
=== Stars: {stars}
=== Alignment: {alignment}
=== Attributes: {attributes}
=== Qualified Servant Classes: {qualified_servant_classes}

== [ Parameters ]
=== Strength: {strength}
=== Endurance: {endurance}
=== Agility: {agility}
=== Mana: {mana}
=== Luck: {luck}
=== NP: {np}

== [ Skills ]
=== Class Skills: {class_skills}
=== Personal Skills: {personal_skills}",
		stars_num = self.stars,
		star = STAR_SOLID,
		class = self.class,
		true_name = self.true_name,
		stars = stars_display,
		alignment = self.alignment,
		attributes = self.attributes,
		qualified_servant_classes = self.qualified_servant_classes,
		strength = self.strength,
		endurance = self.endurance,
		agility = self.agility,
		mana = self.mana,
		luck = self.luck,
		np = self.np,
		class_skills = self.class_skills,
		personal_skills = self.personal_skills);
	}
}

pub fn load(file_name: String) -> Servant {
	let path = format!("{}{}.{}", SERVANT_PATH, file_name, SERVANT_EXTENSION);
	// ^^^ was ./{}/ but now {}. This means the folder input should be ./servants/
	let servant = crate::servant_fs::ro_from_file(path).to_string();
	
	serde_json::from_str(&servant).expect("Unserializing servant failed")
}


pub fn delete(file_path: String) -> io::Result<()> {
	fs::remove_file(file_path).unwrap();
	Ok(())
}

fn get_stars(stars: u32) -> String {
	let usize_stars: usize = stars as usize;
	STAR_SOLID.to_string().repeat(usize_stars)
}