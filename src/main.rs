// Game Basics
//  - Gather Input (guess)
//  - Guess Validation -> ok or err
//  - Check if guess matches answer
//  - Output result - Grey/Yellow/Green Highlights
// Considerations
//  - Six total attempts
//

use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, terminal_size};

use crate::graphics::{display_starting_board, draw_box};

mod graphics;

fn main() {
    // Get the standard input stream.
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

    write!(std_out, "{}{}", BG_BLACK, termion::clear::All).unwrap();

    let (terminal_width, _) = terminal_size().unwrap();
    let start_x = ((terminal_width / 2) as f32) as u16 - 22;

    display_starting_board(&mut std_out, start_x);

    std_out.flush().unwrap();
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
                continue;
            }
            Key::Char('\n') => {
                continue
            }
            Key::Char(c) => {
                if c.is_ascii_alphabetic() {
                    // draw_box(c, &mut std_out, (start_x, 1), FG_GREY, BG_GREY);
                    // draw_box(c, &mut std_out, (start_x + 9, 1), FG_GREY, BG_GREEN);
                    // draw_box(c, &mut std_out, (start_x + 18, 1), FG_GREY, BG_YELLOW);
                    // draw_box(c, &mut std_out, (start_x + 27, 1), FG_GREY, BG_BLACK);
                    // draw_box(c, &mut std_out, (start_x + 36, 1), FG_GREY, BG_YELLOW);
                    // std_out.flush().unwrap();
                }
            }
            Key::Delete => {
                continue;
            }
            _ => (),
        }
    }

    // Flush again.
    std_out.flush().unwrap();
}

// TODO: Parse input (guess)

fn get_input() -> String {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().chars().all(|c| c.is_ascii_alphabetic()) && input.trim().len() == 5 {
            return input.trim().to_owned();
        }
    }
}

// write!(
//     stdout,
//     "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//     // "{}{:}{}{:}{}{:}{}{:}{}{:}{}",
//     cursor::Goto(start_row, start_col),
//     border_color,
//     Box::Top.draw(' '),
//
//     cursor::Goto(start_row, start_col+1),
//     Box::Edge.draw(' '),
//     reset_fg,
//     cursor::Goto(start_row+1, start_col+1),
//     fill_color,
//     Box::Fill.draw(' '),
//     reset_bg,
//     cursor::Goto(start_row+8, start_col+1),
//     border_color,
//     Box::Edge.draw(' '),
//
//     cursor::Goto(start_row, start_col+2),
//     Box::Edge.draw(' '),
//     reset_fg,
//     cursor::Goto(start_row+1, start_col+2),
//     fill_color,
//     text_color,
//     Box::Mid.draw(ch),
//     reset_bg,
//     cursor::Goto(start_row+8, start_col+2),
//     border_color,
//     Box::Edge.draw(' '),
//
//     cursor::Goto(start_row, start_col+3),
//     Box::Edge.draw(' '),
//     reset_fg,
//     cursor::Goto(start_row+1, start_col+3),
//     fill_color,
//     Box::Fill.draw(' '),
//     reset_bg,
//     cursor::Goto(start_row+8, start_col+3),
//     border_color,
//     Box::Edge.draw(' '),
//
//     cursor::Goto(start_row, start_col+4),
//     Box::Bot.draw(' '),
//     reset_fg,
//
//     cursor::Hide,
// )
// let square = get_box();
// write!(
//     stdout,
//     "{}{}{}{}{}{}{}{}{}{}{}",
//     // "{}{:}{}{:}{}{:}{}{:}{}{:}{}",
//     cursor::Goto(start_row, start_col),
//     square[0],
//     cursor::Goto(start_row, start_col+1),
//     square[1],
//     cursor::Goto(start_row, start_col+2),
//     square[2],
//     cursor::Goto(start_row, start_col+3),
//     square[3],
//     cursor::Goto(start_row, start_col+4),
//     square[4],
//     cursor::Hide
// )
// .unwrap();
