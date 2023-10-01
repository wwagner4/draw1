use rand;
use rand::prelude::*;
use speedy2d::color::Color;
use speedy2d::dimen::{Vec2, Vector2};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::{thread, time::Duration};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Obj {
    pos: Vec2,
    dir: Vec2,
}

fn create_ran_obj() -> Obj {
    let mut rng = rand::thread_rng();
    let pos = Vector2 {
        x: rng.gen::<f32>() * 1000.0,
        y: rng.gen::<f32>() * 1000.0,
    };
    let dir = Vector2 {
        x: rng.gen::<f32>() * 10.0 - 5.0,
        y: rng.gen::<f32>() * 10.0 - 5.0,
    };
    Obj { pos, dir }
}

fn mv(obj: &Obj) -> Obj {
    Obj {
        pos: obj.pos + obj.dir,
        dir: obj.dir,
    }
}

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    let h = MyWindowHandler{
        objs : (0..200).map(|_| create_ran_obj()).collect_vec(),
    };
    window.run_loop(h);
}



struct MyWindowHandler {
    objs : Vec<Obj>,

}

impl WindowHandler for MyWindowHandler {

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        for o in &self.objs {
            graphics.draw_circle((o.pos.x, o.pos.y), 10.0, Color::BLUE);
        }

        thread::sleep(Duration::from_millis(50));
        helper.request_redraw();

        self.objs = self.objs.iter()
            .map(|o| mv(o))
            .collect_vec();

    }
}
