use std::io::{stdin, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, terminal_size};

pub mod colors {
    pub const FG_YELLOW: termion::color::Fg<termion::color::Rgb> =
        termion::color::Fg(termion::color::Rgb(181, 159, 59));
    pub const FG_WHITE: termion::color::Fg<termion::color::Rgb> =
        termion::color::Fg(termion::color::Rgb(255, 255, 255));
    pub const FG_BLACK: termion::color::Fg<termion::color::Rgb> =
        termion::color::Fg(termion::color::Rgb(18, 18, 19));
    pub const FG_GREEN: termion::color::Fg<termion::color::Rgb> =
        termion::color::Fg(termion::color::Rgb(83, 141, 78));
    pub const FG_GREY: termion::color::Fg<termion::color::Rgb> =
        termion::color::Fg(termion::color::Rgb(58, 58, 60));

    pub const BG_YELLOW: termion::color::Bg<termion::color::Rgb> =
        termion::color::Bg(termion::color::Rgb(181, 159, 59));
    pub const BG_WHITE: termion::color::Bg<termion::color::Rgb> =
        termion::color::Bg(termion::color::Rgb(255, 255, 255));
    pub const BG_BLACK: termion::color::Bg<termion::color::Rgb> =
        termion::color::Bg(termion::color::Rgb(18, 18, 19));
    pub const BG_GREEN: termion::color::Bg<termion::color::Rgb> =
        termion::color::Bg(termion::color::Rgb(83, 141, 78));
    pub const BG_GREY: termion::color::Bg<termion::color::Rgb> =
        termion::color::Bg(termion::color::Rgb(58, 58, 60));
}

pub mod graphics {
    pub const BOX_TOP: &str = "┏━━━━━━━┓";
    pub const BOX_FILL: &str = "       ";
    pub const BOX_HALF_FILL: &str = "    ";
    pub const BOX_BOT: &str = "┗━━━━━━━┛";
    pub const BOX_EDGE: &str = "┃";
}

use graphics::*;
use colors::*;

pub fn draw_box(
    ch: char,
    stdout: &mut RawTerminal<std::io::Stdout>,
    (start_row, start_col): (u16, u16),
    border_color: color::Fg<termion::color::Rgb>,
    fill_color: color::Bg<termion::color::Rgb>,
) {
    let text_color = FG_WHITE;
    let background_color = BG_BLACK;

    write!(
        stdout,
        "{}{}{}{}",
        // "{}{:}{}{:}{}{:}{}{:}{}{:}{}",
        cursor::Goto(start_row, start_col),
        border_color,
        background_color,
        BOX_TOP
    )
    .unwrap();

    write!(
        stdout,
        "{}{}{}{}{}{}{}{}{}",
        cursor::Goto(start_row, start_col + 1),
        BOX_EDGE,
        cursor::Goto(start_row + 1, start_col + 1),
        fill_color,
        BOX_FILL,
        cursor::Goto(start_row + 8, start_col + 1),
        border_color,
        background_color,
        BOX_EDGE,
    )
    .unwrap();

    write!(
        stdout,
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        cursor::Goto(start_row, start_col + 2),
        BOX_EDGE,
        cursor::Goto(start_row + 1, start_col + 2),
        fill_color,
        BOX_HALF_FILL,
        cursor::Goto(start_row + 4, start_col + 2),
        text_color,
        ch.to_uppercase(),
        cursor::Goto(start_row + 5, start_col + 2),
        BOX_HALF_FILL,
        cursor::Goto(start_row + 8, start_col + 2),
        border_color,
        background_color,
        BOX_EDGE,
    )
    .unwrap();

    write!(
        stdout,
        "{}{}{}{}{}{}{}{}{}",
        cursor::Goto(start_row, start_col + 3),
        BOX_EDGE,
        cursor::Goto(start_row + 1, start_col + 3),
        fill_color,
        BOX_FILL,
        cursor::Goto(start_row + 8, start_col + 3),
        border_color,
        background_color,
        BOX_EDGE,
    )
    .unwrap();

    write!(
        stdout,
        "{}{}{}",
        cursor::Goto(start_row, start_col + 4),
        BOX_BOT,
        cursor::Hide,
    )
    .unwrap();
}

pub fn draw_row(std_out: &mut RawTerminal<std::io::Stdout>, start_x: u16) {
    for row in (1..=26).step_by(5) {
        for col in (0..=36).step_by(9) {
            draw_box(' ', std_out, (start_x + col, row), FG_GREY, BG_BLACK);
        }
    }
}

pub fn display_starting_board(std_out: &mut RawTerminal<std::io::Stdout>, start_x: u16) {
    for row in (1..=26).step_by(5) {
        for col in (0..=36).step_by(9) {
            draw_box(' ', std_out, (start_x + col, row), FG_GREY, BG_BLACK);
        }
    }
}

// TODO: Async function to update middle val & fill terminal background on resize

