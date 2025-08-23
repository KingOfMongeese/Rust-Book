pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}
use prelude::*;
pub trait Drawable {
    fn draw(&self, ctx: &mut BTerm);

    fn run(&mut self);
}

pub struct Rectangle {
    width: i32,
    height: i32,
    pos: Point,
}

impl Rectangle {
    pub fn new(width: i32, height: i32, pos: Point) -> Self {
        Self {
            width,
            height,
            pos,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, ctx: &mut BTerm) {
        for y in self.pos.y..(self.height + self.pos.y) {
            for x in self.pos.x..(self.width + self.pos.x) {
                ctx.set(x, y, RED, RED, to_cp437('B'));
            }
        }
    }

    fn run(&mut self) {
        self.pos.x += 1;
        if self.pos.x > SCREEN_WIDTH {
            self.pos.x = 0;
        }
    }
}

pub struct Square {
    size: i32,
    pos: Point,
}

impl Square {
    pub fn new(size: i32, pos: Point) -> Self {
        Self { size, pos }
    }
}

impl Drawable for Square {
    fn draw(&self, ctx: &mut BTerm) {
        for y in self.pos.y..(self.size + self.pos.y) {
            for x in self.pos.x..(self.size + self.pos.x) {
                ctx.set(x, y, GREEN, GREEN, to_cp437('B'));
            }
        }
    }

    fn run(&mut self) {
        self.pos.x += 1;
        if self.pos.x > SCREEN_WIDTH {
            self.pos.x = 0;
        }
    }
}
