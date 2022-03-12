use std::env::args;
use std::{fs, path};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


struct Doc {
    lines: Vec<String>,
}

#[derive(Debug)]
struct CursorCoordinates {
    pub x: usize,
    pub y: usize,
}

struct Viewer {
    doc: Doc,
    doc_len: usize,
    cursor_pos: CursorCoordinates,
    terminal_size: CursorCoordinates,
    file_name: String,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Please provide file name as argument");
        std::process::exit(0);
    }

    if !path::Path::new(&args[1]).exists() {
        println!("File doesn't exists");
        std::process::exit(0);
    }

    println!("{}", termion::clear::All);
    println!("{}", termion::cursor::Show);
    println!("{}", termion::cursor::Goto(1, 1));

    let mut viewer = Viewer::init(&args[1]);
    viewer.set_pos(1, 1);
    viewer.show_document();
    viewer.run();
}

impl Viewer {
    fn init(file: &str) -> Self {
        let mut doc = Doc { lines: vec![] };
        let file_handle = fs::read_to_string(file).unwrap();

        for doc_line in file_handle.lines() {
            doc.lines.push(doc_line.to_string());
        }

        let doc_len = file_handle.lines().count();
        let size = termion::terminal_size().unwrap();

        Viewer {
            doc: doc,
            cursor_pos: CursorCoordinates {
                x: 1,
                y: doc_len,
            },
            doc_len: doc_len,
            terminal_size: CursorCoordinates {
                x: size.0 as usize,
                y: size.1 as usize,
            },
            file_name: file.into(),
        }
    }

    fn show_document(&mut self) {
        let pos = &self.cursor_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        println!(
            "{}{}Welcome to Imposter viewer\r{}",
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );

        if self.doc_len < self.terminal_size.y {
            for line in 0..self.doc_len {
                println!("{}\r", self.doc.lines[line as usize]);
            }
        } else {
            if pos.y <= self.terminal_size.y {
                for line in 0..self.terminal_size.y - 3 {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            } else {
                for line in pos.y - (self.terminal_size.y - 3)..pos.y {
                    println!("{}\r", self.doc.lines[line as usize]);
                }
            }
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.y - 2) as u16),
        );

        println!(
            "{}Coordinates: (X={}, Y={}), line-count={} Filename: {}{}",
            color::Fg(color::Red),
            old_x,
            old_y,
            self.doc_len,
            self.file_name,
            style::Reset,
        );

        self.set_pos(old_x, old_y);
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.cursor_pos.x = x;
        self.cursor_pos.y = y;

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, (self.cursor_pos.y) as u16),
        );
    }

    fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.show_document();
                }
                Key::Right => {
                    self.inc_x();
                    self.show_document();
                }
                Key::Up => {
                    self.dec_y();
                    self.show_document();
                }
                Key::Down => {
                    self.inc_y();
                    self.show_document();
                }
                Key::Backspace => {
                    self.dec_x();
                }
                _ => {}
            }

            stdout.flush().unwrap();
        }
    }

    fn inc_x(&mut self) {
        if self.cursor_pos.x < self.terminal_size.x {
            self.cursor_pos.x += 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
    }

    fn dec_x(&mut self) {
        if self.cursor_pos.x > 1 {
            self.cursor_pos.x -= 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
    }

    fn inc_y(&mut self) {
        if self.cursor_pos.y < self.doc_len {
            self.cursor_pos.y += 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
    }

    fn dec_y(&mut self) {
        if self.cursor_pos.y > 1 {
            self.cursor_pos.y -= 1;
        }

        println!(
            "{}",
            termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16)
        );
    }
}