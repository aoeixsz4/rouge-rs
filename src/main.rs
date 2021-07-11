use std::io::{self, Read, Write, stdout};
use std::process;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::{async_stdin, AsyncReader};

struct Monster {
    x: u16,
    y: u16,
    glyph: char,
    hp: u16
}

impl Monster {
    pub fn damage(&self) -> u16 {
        return 10;
    }
}

struct Character {
    x: u16,
    y: u16,
    hp: u16
}

impl Character {
    pub fn fight(&mut self, mon: &mut Monster) {
        let player_dmg = self.damage();
        let mon_dmg = mon.damage();
        if player_dmg > mon.hp {
            mon.hp = 0;
        } else {
            mon.hp -= player_dmg;
        }
        if mon.hp > 0 {
            self.hp -= mon_dmg;
            if self.hp < 0 {
                self.hp = 0;
            }
        }
    }

    pub fn damage(&self) -> u16 {
        return 10;
    }

    pub fn move_left(&mut self, mon: &mut Monster) {
        if self.x - 1 != mon.x || self.y != mon.y {
            self.x -= 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_down(&mut self, mon: &mut Monster) {
        if self.x != mon.x || self.y + 1 != mon.y {
            self.y += 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_up(&mut self, mon: &mut Monster) {
        if self.x != mon.x || self.y - 1 != mon.y {
            self.y -= 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_right(&mut self, mon: &mut Monster) {
        if self.x + 1 != mon.x || self.y != mon.y {
            self.x += 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_upleft(&mut self, mon: &mut Monster) {
        if self.x - 1 != mon.x || self.y - 1 != mon.y {
            self.x -= 1;
            self.y -= 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_upright(&mut self, mon: &mut Monster) {
        if self.x + 1 != mon.x || self.y - 1 != mon.y {
            self.x += 1;
            self.y -= 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_downleft(&mut self, mon: &mut Monster) {
        if self.x - 1 != mon.x || self.y + 1 != mon.y {
            self.x -= 1;
            self.y += 1;
        } else {
            self.fight(mon);
        }
    }

    pub fn move_downright(&mut self, mon: &mut Monster) {
        if self.x + 1 != mon.x || self.y + 1 != mon.y {
            self.x += 1;
            self.y += 1;
        } else {
            self.fight(mon);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut read_handle = async_stdin();
    let mut buf:[u8; 1024] = [0; 1024];
    let mut player = Character { x: 10, y: 10, hp: 50 };
    let mut mon = Monster { x: 4, y: 15, glyph: 'd', hp: 10 };
    loop {
        let mut bytes_read = read_handle.read(&mut buf)?;
        print!("{}", termion::clear::All);
        while bytes_read > 0 {
            let byte = buf[bytes_read - 1];
            bytes_read -= 1;
            match byte {
                b'h' => player.move_left(&mut mon),
                b'j' => player.move_down(&mut mon),
                b'k' => player.move_up(&mut mon),
                b'l' => player.move_right(&mut mon),
                b'y' => player.move_upleft(&mut mon),
                b'u' => player.move_upright(&mut mon),
                b'b' => player.move_downleft(&mut mon),
                b'n' => player.move_downright(&mut mon),
                b'q' => process::exit(0),
                _ => (),
            }
        }
        if player.hp > 0 {
            print!("{}@", termion::cursor::Goto(player.x, player.y));
            if mon.hp > 0 {
                print!("{}{}", termion::cursor::Goto(mon.x, mon.y), mon.glyph);
            }
            print!("{}", termion::cursor::Goto(player.x, player.y));
        }
        terminal.flush();
    }
    Ok(())
}
