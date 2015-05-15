#![feature(std_misc)]
extern crate rand;
extern crate rustbox;
use std::process::exit;
use std::time::duration::Duration;
use rustbox::Event::*;
use rustbox::Key::{Esc,Char};
use rustbox::{RustBox,InputMode,RB_NORMAL,Mouse};
use rustbox::Color::*;

static OFFSETS: &'static [(isize,isize)] = &[
    (-1,-1), ( 0,-1), ( 1,-1),
    (-1, 0),          ( 1, 0),
    (-1, 1), ( 0, 1), ( 1, 1)
];

struct GameOfLife {
    width: usize,
    height: usize,
    field: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>
}

impl GameOfLife {
    fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width: width,
            height: height,
            field: vec![vec![false; height]; width],
            next: vec![vec![false; height]; width]
        }
    }

    fn clear(&mut self) {
        self.field = vec![vec![false; self.height()]; self.width()];
    }

    fn get(&self, x: usize, y: usize) -> Option<bool>{
        self.field
            .get(x)
            .and_then(|v| v.get(y).cloned())
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool>{
        self.field
            .get_mut(x)
            .and_then(|v| v.get_mut(y))
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.get_mut(x, y).map(|c| *c = !*c);
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.get_mut(x, y).map(|c| *c = value);
    }

    pub fn set_on(&mut self, x: usize, y: usize) {
        self.set(x, y, true);
    }

    pub fn set_off(&mut self, x: usize, y: usize) {
        self.set(x, y, false);
    }

    pub fn neighbours(&self, x: usize, y: usize) -> usize {
        OFFSETS.into_iter().flat_map(|&(xv, yv)| {
            let (x, y) = (x as isize + xv, y as isize + yv);
            let (x, y) = (x as usize, y as usize);
            self.get(x, y)
                .into_iter()
                .filter(|b| *b == true)
        }).count()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn step(&mut self) {
        let (width, height) = (self.width(), self.height());
        for x in (0..width) {
            for y in (0..height) {
                self.next[x][y] = self.apply_rule(x, y);
            }
        }
        std::mem::swap(&mut self.field, &mut self.next);
    }

    // Implements only Conway's rule for now
    fn apply_rule(&self, x: usize, y: usize) -> bool {
        let cell = self.get(x, y).unwrap_or(false);
        match (cell, self.neighbours(x, y)) {
            (false, 3) => true,
            (true, 2) | (true, 3) => true,
            _ => false
        }
    }
}

fn main() {
    let rustbox = RustBox::init(Default::default()).ok().expect("rustbox init");
    rustbox.set_input_mode(InputMode::EscMouse);

    let (width, height) = (rustbox.width(), rustbox.height());
    let mut game = GameOfLife::new(width, height);
    let mut active = false;

    // Light Weight Spaceship
    game.set_on(1, 2);
    game.set_on(2, 2);
    game.set_on(3, 2);
    game.set_on(4, 2);
    game.set_on(4, 3);
    game.set_on(4, 4);
    game.set_on(0, 3);
    game.set_on(0, 5);
    game.set_on(3, 5);

    // Glider
    game.set_on(0, 9);
    game.set_on(1, 9);
    game.set_on(2, 9);
    game.set_on(2, 8);
    game.set_on(1, 7);

    loop {
        rustbox.clear();

        match rustbox.peek_event(Duration::milliseconds(50), false) {
        //match rustbox.poll_event(false) {
            Ok(KeyEvent(Some(key))) => match key {
                Esc => {drop(rustbox); exit(0)}
                Char(' ') => active = !active,
                Char('c') => game.clear(),
                _ => {}
            },
            Ok(MouseEvent(Mouse::Left, x, y)) => {
                let (x, y) = (x as usize, y as usize);
                game.toggle(x, y);
            }
            _ => {}
        }

        if active {
            game.step();
        }

        for x in (0..width) {
            for y in (0..height) {
                match game.get(x, y) {
                    Some(true) => set_on(&rustbox, x, y),
                    Some(false) => set_off(&rustbox, x, y),
                    None => {}
                }
            }
        }

        rustbox.present();
    }
}

fn set_on(rustbox: &RustBox, x: usize, y: usize) {
    rustbox.print_char(x, y, RB_NORMAL, Black, White, ' ')
}

fn set_off(rustbox: &RustBox, x: usize, y: usize) {
    rustbox.print_char(x, y, RB_NORMAL, White, Black, ' ')
}
