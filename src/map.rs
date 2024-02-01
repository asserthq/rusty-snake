use crate::types::{Coord, Direction};

use std::io;

use rand::Rng;
use crossterm::{queue, cursor::MoveTo, style::Print};

pub struct Map {
    width: u16,
    height: u16,
    food: Option<Coord>
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        Map {
            width,
            height,
            food: None
        }
    }
    
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get_center(&self) -> Coord {
        (self.width / 2, self.height / 2).into()
    }

    pub fn get_nbr(&self, pos: &Coord, dir: &Direction) -> Coord {
        use Direction::*;
        let mut pos = pos.clone();
        match dir {
            Left => {
                if pos.x == self.bounds_min().x {
                    pos.x = self.bounds_max().x;
                }
                else {
                    pos.x -= 1;
                }
            }
            Right => {
                if pos.x == self.bounds_max().x {
                    pos.x = self.bounds_min().x;
                }
                else {
                    pos.x += 1;
                }
            }
            Up => {
                if pos.y == self.bounds_min().y {
                    pos.y = self.bounds_max().y;
                }
                else {
                    pos.y -= 1;
                }
            }
            Down => {
                if pos.y == self.bounds_max().y {
                    pos.y = self.bounds_min().y;
                }
                else {
                    pos.y += 1;
                }
            }
        }
        pos.to_owned()
    }

    pub fn get_food(&self) -> Option<Coord> {
        self.food.to_owned()
    }

    pub fn draw(&self) -> io::Result<()> {
        let border_x = self.width + 1;
        let border_y = self.height + 1;
        
        for x in 0..=border_x {
            queue!(
                io::stdout(),
                MoveTo(x, 0),
                Print('#'),
                MoveTo(x, border_y),
                Print('#')
            )?;
        }

        for y in 0..=border_y {
            queue!(
                io::stdout(),
                MoveTo(0, y),
                Print('#'),
                MoveTo(border_x, y),
                Print('#')
            )?;
        }

        if let Some(food) = self.food.as_ref() {
            queue!(
                io::stdout(),
                MoveTo(food.x, food.y),
                Print('0')
            )?;
        }

        Ok(())
    }

    pub fn respawn_food(&mut self) {
        self.food = Coord::new(
            rand::thread_rng().gen_range(self.bounds_min().x..self.bounds_max().x),
            rand::thread_rng().gen_range(self.bounds_min().y..self.bounds_max().y)
        ).into();
    }

    fn bounds_min(&self) -> Coord {
        (1, 1).into()
    }

    fn bounds_max(&self) -> Coord {
        (self.width, self.height).into()
    }
}