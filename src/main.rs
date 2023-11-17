
fn main() {
    circles::main_circles()
}


mod circles {
    use rand::prelude::*;
    use speedy2d::color::Color;
    use speedy2d::dimen::{UVec2, Vec2, Vector2};
    use speedy2d::window::{WindowHandler, WindowHelper};
    use speedy2d::{Graphics2D, Window};
    use std::{thread, time::Duration};
    use itertools::Itertools;

    #[derive(Clone, Debug)]
    struct Obj {
        pos: Vec2,
        dir: Vec2,
    }

    pub fn main_circles() {
        let width = 1500.0 as u32;
        let height = 1000.0 as u32;
        let window = Window::new_centered("circles", (width, height)).unwrap();
        let screen = UVec2 { x: width, y: height };
        let h = create_window_handler_circles(screen);
        window.run_loop(h);
    
    } 
    
    fn create_window_handler_circles(screen: Vector2<u32>) -> CirclesWindowHandler {
        CirclesWindowHandler {
            screen,
            objs: (0..1000).map(|_| create_ran_obj(&screen)).collect_vec(),
        }
    }
    
    impl WindowHandler for CirclesWindowHandler {
        fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
            helper.set_resizable(false);
            graphics.clear_screen(Color::BLACK);
    
            for o in &self.objs {
                graphics.draw_circle((o.pos.x, o.pos.y), 20.0, Color::from_rgba(1.0, 0.8, 0.1, 0.1));
            }
    
            thread::sleep(Duration::from_millis(100));
            helper.request_redraw();
    
            self.objs = self.objs.iter()
                .map(|o| mv(o, &self.screen))
                .collect_vec();
        }
    }
    
    fn create_ran_obj(screen: &UVec2) -> Obj {
        let mut rng = rand::thread_rng();
        let pos = Vector2 {
            x: rng.gen::<f32>() * screen.x as f32,
            y: rng.gen::<f32>() * screen.y as f32,
        };
        let dir = Vector2 {
            x: rng.gen::<f32>() * 10.0 - 5.0,
            y: rng.gen::<f32>() * 10.0 - 5.0,
        };
        Obj { pos, dir }
    }
    
    fn in_max(value: f32, max: u32) -> f32 {
        if value > 0.0 {
            value % max as f32
        } else {
            max as f32 - (value.abs() / max as f32).abs()
        }
    }
    
    fn mv(obj: &Obj, screen: &UVec2) -> Obj {
        let x = in_max(obj.pos.x + obj.dir.x, screen.x);
        let y = in_max(obj.pos.y + obj.dir.y, screen.y);
        Obj {
            pos: Vec2 { x, y },
            dir: obj.dir,
        }
    }
    
    struct CirclesWindowHandler {
        screen: UVec2,
        objs: Vec<Obj>,
    
    }
}
