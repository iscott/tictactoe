extern crate cfonts;
use cfonts::{Align, BgColors, Fonts, Options};

#[derive(Debug, Clone, Copy)]
enum CellState {
    Empty,
    Ex,
    Oh,
}

type State = [[CellState; 3]; 3];

fn main() {
    let thing: f64 = 0.5;
    let thong: f32 = 0.5;
    // println!("Hello, world! NOT READ ONLY ANYMOOOAARRR {:?}", thing>thong);

    let app_state: State = [
		[CellState::Empty,CellState::Empty,CellState::Empty],
		[CellState::Ex,CellState::Ex,CellState::Empty],
		[CellState::Empty,CellState::Oh,CellState::Empty],
	];

    let header = draw(&app_state);
    println!("{}", header);
}

fn draw(state: &State) -> String {
    // Return string
    let logo = cfonts::render(Options {
        text: String::from("Tic Tac Toe"),
        gradient: vec![String::from("#ff0000"), String::from("#ff0000")],
        background: BgColors::Black,
        font: Fonts::FontBlock,
        align: Align::Center,
        ..Options::default()
    });

	let mut output = logo.text.clone();
	output += &format!("\n\n ╔═══╦═══╦═══╗\n ║");

	for (i, line) in state.iter().enumerate() {
		for cell in line.iter() {
			let this_cell = match cell {
				CellState::Empty =>  "  ".to_string(),
				CellState::Ex =>  " X".to_string(),
				CellState::Oh => " O".to_string(),
			};
			output += &format!("{} ║", this_cell);
		}
		if i != 2 {
			output += &format!("\n ╠═══╬═══╬═══╣\n ║");
		} else {
			output += &format!("\n");
		}
	}
	output += &format!(" ╚═══╩═══╩═══╝");

    // "
    // ╔═══╦═══╦═══╗
    // ║ X ║   ║   ║
    // ╠═══╬═══╬═══╣
    // ║ X ║ O ║ X ║
    // ╠═══╬═══╬═══╣
    // ║ X ║ O ║   ║
    // ╚═══╩═══╩═══╝
    // "

    output
}

// 'c' // <-- char
// "str" // <-- str
// let my_string = "string".to_string() // <-- String
// &my_string // <-- str (& reference)
// *reference // <-- dereference (compiler gets value)

// TODO: Macros
