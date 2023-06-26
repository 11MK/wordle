// Game Basics
//  - Gather Input (guess)
//  - Guess Validation -> ok or err
//  - Check if guess matches answer
//  - Output result - Grey/Yellow/Green Highlights
// Considerations
//  - Six total attempts
//

// use std::io::{stdin, stdout, Read, Write};
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::{IntoRawMode, RawTerminal};
// use termion::{color, cursor, terminal_size};

use std::io::{stdin, stdout, Write};
use termion::{raw::IntoRawMode, input::TermRead, terminal_size};
use termion::event::Key;
use termion::cursor;

use crate::game::{Wordle, Letter, Character};
use crate::graphics::colors;

mod graphics;
mod game;

// Write a function that takes a 


fn main() {
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut std_out = stdout().into_raw_mode().unwrap();
    write!(
        std_out,
        "{}{}{}",
        cursor::Goto(1, 2),
        termion::clear::All,
        cursor::Hide
    )
    .unwrap();

    write!(std_out, "{}{}", colors::BG_BLACK, termion::clear::All).unwrap();
    std_out.flush().unwrap();


    let (terminal_width, _) = terminal_size().unwrap();
    let col_start = ((terminal_width / 2) as f32) as u16 - 22;
    let row = 1;

    let mut wordle_game = Wordle::initialize();
    for c in stdin.keys() {
        match c.unwrap() {
            // Exit.
            Key::Esc => {
                write!(
                    std_out,
                    "{}{}{}",
                    cursor::Goto(1, 1),
                    termion::clear::All,
                    cursor::Restore,
                )
                .unwrap();
                break;
            }
            Key::Backspace => {
                wordle_game.delete_last_char();
                let updated_letters = wordle_game.get_guess_letters();
                graphics::draw_row(&mut std_out, col_start, row, updated_letters)
            }
            Key::Char('\n') => {
                match wordle_game.validate_guess() {
                    true => {
                        wordle_game.submit_guess();
                        let updated_letters = wordle_game.get_guess_letters();
                        graphics::draw_row(&mut std_out, col_start, row, updated_letters)
                    },
                    false => print!("INVALID GUESS"),
                }
            }
            Key::Char(c) => {
                if c.is_ascii_alphabetic() {
                    let letter = Letter { ascii: c, state: Character::None, first_blank: false };
                    wordle_game.update_guess(letter);
                    let updated_letters = wordle_game.get_guess_letters();
                    graphics::draw_row(&mut std_out, col_start, row, updated_letters)
                }
            }
            Key::Delete => {
                continue;
            }
            _ => (),
        }
    }
}

// fn main() {
//     // Get the standard input stream.
//     let stdin = stdin();
//     // Get the standard output stream and go to raw mode.
//     let mut std_out = stdout().into_raw_mode().unwrap();
//     write!(
//         std_out,
//         "{}{}{}",
//         cursor::Goto(1, 2),
//         termion::clear::All,
//         cursor::Hide
//     )
//     .unwrap();
//
//     write!(std_out, "{}{}", BG_BLACK, termion::clear::All).unwrap();
//
//     let (terminal_width, _) = terminal_size().unwrap();
//     let start_x = ((terminal_width / 2) as f32) as u16 - 22;
//
//     display_starting_board(&mut std_out, start_x);
//
//     std_out.flush().unwrap();
//     for c in stdin.keys() {
//         match c.unwrap() {
//             // Exit.
//             Key::Esc => {
//                 write!(
//                     std_out,
//                     "{}{}{}",
//                     cursor::Goto(1, 1),
//                     termion::clear::All,
//                     cursor::Restore,
//                 )
//                 .unwrap();
//                 break;
//             }
//             Key::Backspace => {
//                 continue;
//             }
//             Key::Char('\n') => {
//                 continue
//             }
//             Key::Char(c) => {
//                 if c.is_ascii_alphabetic() {
//                     // draw_box(c, &mut std_out, (start_x, 1), FG_GREY, BG_GREY);
//                     // draw_box(c, &mut std_out, (start_x + 9, 1), FG_GREY, BG_GREEN);
//                     // draw_box(c, &mut std_out, (start_x + 18, 1), FG_GREY, BG_YELLOW);
//                     // draw_box(c, &mut std_out, (start_x + 27, 1), FG_GREY, BG_BLACK);
//                     // draw_box(c, &mut std_out, (start_x + 36, 1), FG_GREY, BG_YELLOW);
//                     // std_out.flush().unwrap();
//                 }
//             }
//             Key::Delete => {
//                 continue;
//             }
//             _ => (),
//         }
//     }
//
//     // Flush again.
//     std_out.flush().unwrap();
// }


