use std::io::{self, Read, Write, stdout};
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::{async_stdin, AsyncReader};

struct Character {
    x: u16,
    y: u16
}

impl Character {
    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_upleft(&mut self) {
        self.move_up();
        self.move_left();
    }

    pub fn move_upright(&mut self) {
        self.move_up();
        self.move_right();
    }

    pub fn move_downleft(&mut self) {
        self.move_down();
        self.move_left();
    }

    pub fn move_downright(&mut self) {
        self.move_down();
        self.move_right();
    }
}

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut read_handle = async_stdin();
    let mut buf:[u8; 1024] = [0; 1024];
    let mut player = Character { x: 10, y: 10 };
    loop {
        let mut bytes_read = read_handle.read(&mut buf)?;
        print!("{}", termion::clear::All);
        while bytes_read > 0 {
            let byte = buf[bytes_read - 1];
            bytes_read -= 1;
            match byte {
                b'h' => player.move_left(),
                b'j' => player.move_down(),
                b'k' => player.move_up(),
                b'l' => player.move_right(),
                b'y' => player.move_upleft(),
                b'u' => player.move_upright(),
                b'b' => player.move_downleft(),
                b'n' => player.move_downright(),
                _ => (),
            }
        }
        print!("{}@", termion::cursor::Goto(player.x, player.y));
        print!("{}", termion::cursor::Goto(player.x, player.y));
        terminal.flush();
    }
    Ok(())
}
