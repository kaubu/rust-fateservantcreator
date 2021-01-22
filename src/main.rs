const APP_VERSION: &str = "0.1.1";

fn main() {
	println!("[ Fate/Servant Creator ver. {} ]
[ Written in Rust by github.com/kaubu ]", APP_VERSION);

	fateservant::init_menu();
}
