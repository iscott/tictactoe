//! Hello world
extern crate cfonts;
extern crate termion;

use std::cell::Cell;
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Ex,
    Oh,
    Me(PrevState),
}

enum GameResult {
    WinEx,
    WinOh,
    Tie,
    OnGoing,
}

type State = [[CellState; 3]; 3];

fn main() {
    // let thing: f64 = 0.5;
    // let thong: f32 = 0.5;
    // println!("Hello, world! NOT READ ONLY ANYMOOOAARRR {:?}", thing>thong);

    let mut app_state: State = [
        [
            CellState::Me(PrevState::Empty),
            CellState::Empty,
            CellState::Empty,
        ],
        [CellState::Empty; 3],
        [CellState::Empty; 3],
    ];
    let mut turn = PrevState::Ex;
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
            // KILL SWITCH
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
            // MAKING YOU MOVE
            Key::Char(' ') | Key::Char('\n') => {
                println!("Spacebar or Enter key pressed");

                let below_state = match app_state[position.0][position.1] {
                    CellState::Me(prev) => prev,
                    _ => unreachable!("position and state is out of sync"),
                };
                if below_state == PrevState::Empty {
                    app_state[position.0][position.1] = CellState::Me(turn);
                    turn = swap_turn(&turn);
                } else { /* we ignore this illegal move */
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

        // draw the board
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

        // check for end of game


    }
}

fn printing(stdout: &mut dyn io::Write, thing: String) {
    write!(stdout, "{}", thing.replace("\n", "\r\n")).unwrap();
    stdout.flush().unwrap();
}

// TODO (Hi 👋 future Ira and Dom)
// (0,0), (0,1), (0,2)
// (1,0), (1,1), (1,2)
// (2,0), (2,1), (2,2)

// (0,0), (1,0), (2,0)
// (0,1), (1,1), (2,1)
// (0,2), (1,2), (2,2)

// (0,0), (1,1), (2,2)
// (0,2), (1,1), (2,0)

// Count empties -> no empty = end of game
// GameResult::WinEx
// GameResult::WinOh
// GameResult::Tie
// GameResult::OnGoing

fn game_status(app_state: &State) -> GameResult {
    let mut result = GameResult::OnGoing;
    let win_conditions = [
        [(0,0), (0,1), (0,2)],
        [(1,0), (1,1), (1,2)],
        [(2,0), (2,1), (2,2)],
        [(0,0), (1,0), (2,0)],
        [(0,1), (1,1), (2,1)],
        [(0,2), (1,2), (2,2)],
        [(0,0), (1,1), (2,2)],
        [(0,2), (1,1), (2,0)],
    ];
    
    for wins in win_conditions {
        if app_state[wins[0].0] == app_state[wins[0].1] &&
            app_state[wins[0].0] == app_state[wins[1].0] &&
            app_state[wins[0].0] == app_state[wins[1].1] &&
            app_state[wins[0].0] == app_state[wins[2].0] &&
            app_state[wins[0].0] == app_state[wins[3].1] {
            // &&
            // app_state[wins[0].0] != CellState::Empty {
                result = match app_state[wins[0].0] {
                    CellState::Ex => GameResult::WinEx,
                    CellState::Oh => GameResult::WinOh,
                }
                break;
            }
    }

    result
}

#[test]
fn test_game_status() {
    let mut state = [[State::Empty; 3]; 3];
    assert_eq!(game_status(&state), GameResult::OnGoing);
    // Change state
    state[0][0] = CellState::Ex;
    state[1][0] = CellState::Ex;
    state[2][0] = CellState::Ex;
    assert_eq!(game_status(&state), GameResult::WinEx);
}

fn swap_turn(turn: &PrevState) -> PrevState {
    match *turn {
        PrevState::Ex => PrevState::Oh,
        PrevState::Oh => PrevState::Ex,
        PrevState::Empty => PrevState::Empty,
    }
}

#[test]
fn swap_turn_works() {
    assert_eq!(swap_turn(&PrevState::Oh), PrevState::Ex);
    assert_eq!(swap_turn(&PrevState::Ex), PrevState::Oh);
    assert_eq!(swap_turn(&PrevState::Empty), PrevState::Empty);
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
                CellState::Empty => "   ".to_string(),
                CellState::Ex => " X ".to_string(),
                CellState::Oh => " O ".to_string(),
                CellState::Me(x) => {
                    // TODO
                    match x {
                        PrevState::Empty => "\x1b[42m\x1b[30m   \x1b[39m\x1b[49m".to_string(),
                        PrevState::Ex => "\x1b[42m\x1b[30m X \x1b[39m\x1b[49m".to_string(),
                        PrevState::Oh => "\x1b[42m\x1b[30m O \x1b[39m\x1b[49m".to_string(),
                    }
                }
            };
            output += &format!("{}║", this_cell);
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
