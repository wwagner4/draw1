use rand;
use rand::prelude::*;
use speedy2d::color::Color;
use speedy2d::dimen::{Vec2, Vector2};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::{thread, time::Duration};

struct Obj {
    pos: Vec2,
    dir: Vec2,
}

fn mv(obj: &Obj) -> Obj {
    Obj {
        pos: obj.pos + obj.dir,
        dir: obj.dir,
    }
}

fn create_ran_obj() -> Obj {
    let mut rng = rand::thread_rng();
    let pos = Vector2 {
        x: rng.gen::<f32>() * 100.0,
        y: rng.gen::<f32>() * 100.0,
    };
    let dir = Vector2 {
        x: rng.gen::<f32>() * 10.0,
        y: rng.gen::<f32>() * 10.0,
    };
    Obj { pos, dir }
}

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler {});
}

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        graphics.draw_line((0.0, 0.0), (50.0, 50.0), 12.0, Color::GREEN);
        thread::sleep(Duration::from_millis(50));
        helper.request_redraw();
    }
}
