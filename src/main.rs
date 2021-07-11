use std::io::{self, Read, Write, stdout};
use std::process;
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

struct Monster {
    x: u16,
    y: u16,
    glyph: char
}

impl Character {
    pub fn move_left(&mut self, mon: &Monster) {
        if self.x - 1 != mon.x || self.y != mon.y {
            self.x -= 1;
        }
    }

    pub fn move_down(&mut self, mon: &Monster) {
        if self.x != mon.x || self.y + 1 != mon.y {
            self.y += 1;
        }
    }

    pub fn move_up(&mut self, mon: &Monster) {
        if self.x != mon.x || self.y - 1 != mon.y {
            self.y -= 1;
        }
    }

    pub fn move_right(&mut self, mon: &Monster) {
        if self.x + 1 != mon.x || self.y != mon.y {
            self.x += 1;
        }
    }

    pub fn move_upleft(&mut self, mon: &Monster) {
        if self.x - 1 != mon.x || self.y - 1 != mon.y {
            self.x -= 1;
            self.y -= 1;
        }
    }

    pub fn move_upright(&mut self, mon: &Monster) {
        if self.x + 1 != mon.x || self.y - 1 != mon.y {
            self.x += 1;
            self.y -= 1;
        }
    }

    pub fn move_downleft(&mut self, mon: &Monster) {
        if self.x - 1 != mon.x || self.y + 1 != mon.y {
            self.x -= 1;
            self.y += 1;
        }
    }

    pub fn move_downright(&mut self, mon: &Monster) {
        if self.x + 1 != mon.x || self.y + 1 != mon.y {
            self.x += 1;
            self.y += 1;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut read_handle = async_stdin();
    let mut buf:[u8; 1024] = [0; 1024];
    let mut player = Character { x: 10, y: 10 };
    let mon = Monster { x: 4, y: 15, glyph: 'd' };
    loop {
        let mut bytes_read = read_handle.read(&mut buf)?;
        print!("{}", termion::clear::All);
        while bytes_read > 0 {
            let byte = buf[bytes_read - 1];
            bytes_read -= 1;
            match byte {
                b'h' => player.move_left(&mon),
                b'j' => player.move_down(&mon),
                b'k' => player.move_up(&mon),
                b'l' => player.move_right(&mon),
                b'y' => player.move_upleft(&mon),
                b'u' => player.move_upright(&mon),
                b'b' => player.move_downleft(&mon),
                b'n' => player.move_downright(&mon),
                b'q' => process::exit(0),
                _ => (),
            }
        }
        print!("{}@", termion::cursor::Goto(player.x, player.y));
        print!("{}{}", termion::cursor::Goto(mon.x, mon.y), mon.glyph);
        print!("{}", termion::cursor::Goto(player.x, player.y));
        terminal.flush();
    }
    Ok(())
}
