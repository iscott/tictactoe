//! Hello world
extern crate cfonts;
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use cfonts::{Align, BgColors, Fonts, Options};

#[derive(Debug, Clone, Copy)]
enum PrevState {
    Empty,
    Ex,
    Oh,
}

#[derive(Debug, Clone, Copy)]
enum CellState {
    Empty,
    Ex,
    Oh,
    Me(PrevState),
}

type State = [[CellState; 3]; 3];

fn main() {
    // let thing: f64 = 0.5;
    // let thong: f32 = 0.5;
    // println!("Hello, world! NOT READ ONLY ANYMOOOAARRR {:?}", thing>thong);

    let app_state: State = [
		[CellState::Me(PrevState::Empty),CellState::Empty,CellState::Empty],
		[CellState::Ex,CellState::Ex,CellState::Empty],
		[CellState::Empty,CellState::Oh,CellState::Empty],
	];

    // Return string
    let logo = cfonts::render(Options {
        text: String::from("Tic Tac Toe"),
        gradient: vec![String::from("#ff0000"), String::from("#ff0000")],
        background: BgColors::Black,
        font: Fonts::FontBlock,
        align: Align::Center,
        ..Options::default()
    });

    let header = draw(&app_state);

    // let size = termion::terminal_size();

	// if let Ok((width, height)) = size {
	// 	if width < min_width || height < min_height {
	// 		panic!("\r\n\r\n{}This terminal is not big enough width width:{} height:{}\r\nTo play Battlefield you need at least width:{} height:{}{}\r\n\r\n", termion::color::Fg(termion::color::Red), width, height, min_width, min_height, termion::color::Fg(termion::color::Reset));
	// 	}
	// } else {
	// 	panic!("The size of the terminal can't be determined");
	// }

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::color::Bg(termion::color::Black), termion::clear::All).unwrap();
	stdout.flush().unwrap();
    
    write!(
		stdout,
		"{}{}{}{}{}{}{}",
		termion::clear::AfterCursor,
		termion::cursor::Goto(1, 2),
		termion::color::Fg(termion::color::White),
		termion::cursor::Hide,
		logo.text,
		header,
		termion::cursor::Save
	)
	.unwrap();
	stdout.flush().unwrap();


    for key in stdin().keys() {
		match key.unwrap() {
			Key::Esc | Key::Char('q') => {
				write!(stdout, "{}{}", termion::cursor::Restore, termion::cursor::Show).unwrap();
				stdout.flush().unwrap();
				termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
				std::process::exit(0);
			}
			Key::Char('r') => {}
            // MOVEMENT
			Key::Left => {
				// do work
			}
			Key::Right => {
				// do work
			}
			Key::Up => {
				// do work
			}
			Key::Down => {
				// do work
            }
            _ => {}
        }
        // draw everything
    }

}

/// This draw function draws our state to a human display thongo
/// ```rust
/// let state = [[CellState::Empty; 3]; 3];
/// let result = "\n\n ╔═══╦═══╦═══╗
/// ║   ║   ║   ║
/// ╠═══╬═══╬═══╣
/// ║   ║   ║   ║
/// ╠═══╬═══╬═══╣
/// ║   ║   ║   ║
/// ╚═══╩═══╩═══╝";
/// assert_eq!(draw(&state), result);
/// ```
fn draw(state: &State) -> String {
	let mut output = String::new();
	output += &format!("\n\n ╔═══╦═══╦═══╗\n ║");

	for (i, line) in state.iter().enumerate() {
		for cell in line.iter() {
			let this_cell = match cell {
				CellState::Empty =>  "  ".to_string(),
				CellState::Ex =>  " X".to_string(),
				CellState::Oh => " O".to_string(),
                CellState::Me(_) => " •".to_string(),
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

#[test]
fn draw_test() {
    let mut state = [[CellState::Empty; 3]; 3];
    let mut result = "\n\n ╔═══╦═══╦═══╗
 ║   ║   ║   ║
 ╠═══╬═══╬═══╣
 ║   ║   ║   ║
 ╠═══╬═══╬═══╣
 ║   ║   ║   ║
 ╚═══╩═══╩═══╝";
    assert_eq!(draw(&state), result);

    state = [[CellState::Empty; 3]; 3];
    state[1][1] = CellState::Ex;
    state[1][2] = CellState::Oh;
    result = "\n\n ╔═══╦═══╦═══╗
 ║   ║   ║   ║
 ╠═══╬═══╬═══╣
 ║   ║ X ║ O ║
 ╠═══╬═══╬═══╣
 ║   ║   ║   ║
 ╚═══╩═══╩═══╝";
    assert_eq!(draw(&state), result);
}
// 'c' // <-- char
// "str" // <-- str
// let my_string = "string".to_string() // <-- String
// &my_string // <-- str (& reference)
// *reference // <-- dereference (compiler gets value)

// TODO: Macros
