use trait_objects::{Drawable, Rectangle, Square, prelude::*};

struct State {
    drawables: Vec<Box<dyn Drawable>>,
}

impl State {
    fn new() -> Self {
        Self {
            drawables: vec![
                Box::new(Rectangle::new(3, 15, Point::new(0, 4))),
                Box::new(Square::new(3, Point::new(0, 25))),
            ],
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.drawables.iter_mut().for_each(|drawable| {
            drawable.run();
            drawable.draw(ctx);
        });
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().build()?;

    main_loop(context, State::new())
}
