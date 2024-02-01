use crate::types::{Coord, Direction};
use crate::map::Map;

use std::collections::LinkedList;
use std::io;

use crossterm::event::KeyCode;
use crossterm::{queue, cursor::MoveTo, style::Print};

pub struct Snake<'a> {
    elements: LinkedList<Coord>,
    dir: Direction,
    eated: bool,
    map: &'a mut Map
}

impl<'a> Snake<'a> {
    pub fn new(map: &'a mut Map, pos: Coord, dir: Direction, len: u16) -> Result<Self, String> {
        if dir.is_vertical() && len > map.height() || 
        !dir.is_vertical() && len > map.width() {
            return Err("Too big snake length".to_string());
        }

        let mut elements = LinkedList::new();
        let mut pos = pos;
        for _ in 0..len {
            elements.push_front(pos.clone());
            pos = map.get_nbr(&pos, &dir);
        }

        Ok(Snake {
            elements,
            dir,
            eated: false,
            map
        })
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn update_dir(&mut self, key: KeyCode) -> bool {
        let new_dir = match key {
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            _ => return false
        };

        if self.dir.can_change_to(&new_dir) {
            self.dir = new_dir;
            true
        }
        else {
            false
        }
    }

    pub fn travel(&mut self) -> bool {
        let new_el = self.map.get_nbr(self.elements.front().unwrap(), &self.dir);

        if self.elements.contains(&new_el) {
            return false;
        }

        if let Some(food) = self.map.get_food() {
            if new_el == food {
                self.eated = true;
                self.map.respawn_food();
            }
        }

        if self.eated {
            self.eated = false;
        } else {
            self.elements.pop_back();
        }
        self.elements.push_front(new_el);

        true
    }

    pub fn draw(&self) -> io::Result<()> {
        self.map.draw()?;

        if let Some(head) = self.elements.front() {
            queue!(
                io::stdout(),
                MoveTo(head.x, head.y),
                Print('*')
            )?;
        }

        for el in self.elements.iter().skip(1) {
            queue!(
                io::stdout(),
                MoveTo(el.x, el.y),
                Print('o')
            )?;
        }

        Ok(())
    }
}