use crate::map::Map;
use crate::snake::Snake;
use crate::types::Direction;

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::cursor;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    execute, queue,
    style::Print,
    terminal,
};

pub mod map;
pub mod snake;
pub mod types;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    queue!(stdout, cursor::Hide)?;

    let mut map = Map::new(16, 8);
    map.respawn_food();
    let mut snake = Snake::new(&mut map, (4, 4).into(), Direction::Right, 3).unwrap();

    let mut timer = Instant::now();
    let period = Duration::from_millis(200);

    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(event) = read()? {
                snake.update_dir(event.code);
            }
        }
        if timer.elapsed() > period {
            if !snake.travel() {
                break;
            }

            queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
            snake.draw()?;
            stdout.flush()?;

            timer = Instant::now();
        }
    }

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        MoveTo(0, 0),
        Print(format!("Game Over. Total length: {}", snake.length()))
    )?;

    Ok(())
}
