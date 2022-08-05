extern crate cfonts;
use cfonts::{render, Options, BgColors, Fonts, Align};

fn main() {
	let thing: f64 = 0.5;
	let thong: f32 = 0.5;
	// println!("Hello, world! NOT READ ONLY ANYMOOOAARRR {:?}", thing>thong);
	let header = draw();
	println!("{}", header);
}

fn draw() -> String {
	// Return string
	let output = render(Options {
		text: String::from("Tic Tac Toe"),
		gradient: vec![String::from("#ff0000"),String::from("#ff0000")],
		background: BgColors::Black,
		font: Fonts::FontBlock,
		align: Align::Center,
		..Options::default()
	});

	output.text
}

// 'c' // <-- char
// "str" // <-- str
// let my_string = "string".to_string() // <-- String
// &my_string // <-- str (& reference)
// *reference // <-- dereference (compiler gets value)

// TODO: Macros