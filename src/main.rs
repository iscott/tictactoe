//! Hello world
extern crate cfonts;
extern crate termion;

use std::io;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use cfonts::{Align, BgColors, Fonts, Options};

#[derive(Debug, Clone, Copy, PartialEq)]
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

    let mut app_state: State = [
        [
            CellState::Me(PrevState::Empty),
            CellState::Ex,
            CellState::Empty,
        ],
        [CellState::Ex, CellState::Ex, CellState::Empty],
        [CellState::Empty, CellState::Oh, CellState::Empty],
    ];
    let mut position = (0, 0);
    let mut prev_pos = (0, 0);

    // Return string
    let logo = cfonts::render(Options {
        text: String::from("Tic Tac Toe"),
        gradient: vec![String::from("#ff0000"), String::from("#ff0000")],
        background: BgColors::Black,
        font: Fonts::FontBlock,
        align: Align::Center,
        ..Options::default()
    });

    // let size = termion::terminal_size();

    // if let Ok((width, height)) = size {
    // 	if width < min_width || height < min_height {
    // 		panic!("\r\n\r\n{}This terminal is not big enough width width:{} height:{}\r\nTo play Battlefield you need at least width:{} height:{}{}\r\n\r\n", termion::color::Fg(termion::color::Red), width, height, min_width, min_height, termion::color::Fg(termion::color::Reset));
    // 	}
    // } else {
    // 	panic!("The size of the terminal can't be determined");
    // }

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}",
        termion::color::Bg(termion::color::Black),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();

    printing(
        &mut stdout,
        format!(
            "{}{}{}{}{}{}{}",
            termion::clear::AfterCursor,
            termion::cursor::Goto(1, 1),
            termion::color::Fg(termion::color::White),
            termion::cursor::Hide,
            logo.text,
            draw(&app_state),
            termion::cursor::Save
        ),
    );

    for key in stdin().keys() {
        prev_pos = position.clone();

        // clone
        match key.unwrap() {
            Key::Esc | Key::Char('q') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Restore,
                    termion::cursor::Show
                )
                .unwrap();
                stdout.flush().unwrap();
                termion::raw::RawTerminal::suspend_raw_mode(&stdout).unwrap();
                print!("\n");
                std::process::exit(0);
            }
            Key::Char(' ') | Key::Char('\n') => {
                println!("Spacebar or Enter key pressed");

                let below_state = match app_state[position.0][position.1] {
                    CellState::Me(prev) => prev,
                    _ => unreachable!("position and state is out of sync"),
                };
                if below_state == PrevState::Empty {
                    println!("????");
                } else { /* we ignore this illegal move */ }
            }
            // MOVEMENT
            Key::Left => {
                if position.1 == 0 {
                    position.1 = 2;
                } else {
                    position.1 -= 1;
                }
            }
            Key::Right => {
                if position.1 < 2 {
                    position.1 += 1;
                } else {
                    position.1 = 0;
                }
            }
            Key::Up => {
                if position.0 == 0 {
                    position.0 = 2;
                } else {
                    position.0 -= 1;
                }
            }
            Key::Down => {
                if position.0 == 2 {
                    position.0 = 0;
                } else {
                    position.0 += 1;
                }
            }
            _ => {}
        }

        let prev_state = match app_state[prev_pos.0][prev_pos.1] {
            CellState::Me(prev) => prev,
            _ => unreachable!("position and state is out of sync"),
        };
        app_state[prev_pos.0][prev_pos.1] = match prev_state {
            PrevState::Empty => CellState::Empty,
            PrevState::Ex => CellState::Ex,
            PrevState::Oh => CellState::Oh,
        };
        let next_state = match app_state[position.0][position.1] {
            CellState::Empty => PrevState::Empty,
            CellState::Ex => PrevState::Ex,
            CellState::Oh => PrevState::Oh,
            _ => unreachable!(),
        };
        app_state[position.0][position.1] = CellState::Me(next_state);

        //
        printing(
            &mut stdout,
            format!(
                "{}{}{}{}{}{}{}",
                termion::clear::AfterCursor,
                termion::cursor::Goto(1, 1),
                termion::color::Fg(termion::color::White),
                termion::cursor::Hide,
                logo.text,
                draw(&app_state),
                termion::cursor::Save
            ),
        );
    }
}

fn printing(stdout: &mut dyn io::Write, thing: String) {
    write!(stdout, "{}", thing.replace("\n", "\r\n")).unwrap();
    stdout.flush().unwrap();
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
                CellState::Empty => "  ".to_string(),
                CellState::Ex => " X".to_string(),
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
