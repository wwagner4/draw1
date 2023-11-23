use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...


    // circles::main();
    // leafes::main();
}

mod leafes {
    use speedy2d::color::Color;
    use speedy2d::dimen::{UVec2, Vec2, Vector2};
    use speedy2d::image::ImageSmoothingMode;
    use speedy2d::image::{self, ImageHandle};
    use speedy2d::window::{WindowHandler, WindowHelper};
    use speedy2d::{Graphics2D, Window};
    use std::{thread, time::Duration};

    struct MyWindowHandler {
        screen: UVec2,
        cnt: u32,
        image: Option<ImageHandle>,
    }

    pub fn main() {
        let width = 1000.0 as u32;
        let height = 1000.0 as u32;
        let window = Window::new_centered("leafes", (width, height)).unwrap();
        let screen = UVec2 {
            x: width,
            y: height,
        };
        let h = create_window_handler(screen);
        window.run_loop(h);
    }

    fn create_window_handler(screen: Vector2<u32>) -> MyWindowHandler {
        MyWindowHandler {
            screen,
            image: None::<ImageHandle>,
            cnt: 0,
        }
    }

    impl WindowHandler for MyWindowHandler {
        fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
            helper.set_resizable(false);

            if self.image.is_none() {
                let image = graphics
                    .create_image_from_file_path(
                        None,
                        ImageSmoothingMode::NearestNeighbor,
                        "data/b1.png",
                    )
                    .unwrap();
                helper.set_size_pixels(*image.size());
                self.image = Some(image);
            }
            graphics.clear_screen(Color::WHITE);
            let x = (self.cnt as f32) * 0.3;
            let y = (self.cnt as f32) * 0.1;
            let pos = Vec2 { x, y };
            // println!("pos {:?}", pos);
            graphics.draw_image(pos, self.image.as_ref().unwrap());
            self.cnt += 1;
            // thread::sleep(Duration::from_millis(1));
            helper.request_redraw();
        }
    }
}

mod circles {
    use itertools::Itertools;
    use rand::prelude::*;
    use speedy2d::color::Color;
    use speedy2d::dimen::{UVec2, Vec2, Vector2};
    use speedy2d::window::{WindowHandler, WindowHelper};
    use speedy2d::{Graphics2D, Window};
    use std::{thread, time::Duration};

    #[derive(Clone, Debug)]
    struct Obj {
        pos: Vec2,
        dir: Vec2,
    }

    struct CirclesWindowHandler {
        screen: UVec2,
        objs: Vec<Obj>,
    }

    pub fn main() {
        let width = 1500.0 as u32;
        let height = 1000.0 as u32;
        let window = Window::new_centered("circles", (width, height)).unwrap();
        let screen = UVec2 {
            x: width,
            y: height,
        };
        let h = create_window_handler_circles(screen);
        window.run_loop(h);
    }

    fn create_window_handler_circles(screen: Vector2<u32>) -> CirclesWindowHandler {
        CirclesWindowHandler {
            screen,
            objs: (0..100).map(|_| create_ran_obj(&screen)).collect_vec(),
        }
    }

    impl WindowHandler for CirclesWindowHandler {
        fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
            helper.set_resizable(false);
            graphics.clear_screen(Color::BLACK);

            for o in &self.objs {
                graphics.draw_circle(
                    (o.pos.x, o.pos.y),
                    20.0,
                    Color::from_rgba(1.0, 0.8, 0.1, 0.1),
                );
            }

            //  thread::sleep(Duration::from_millis(10));
            helper.request_redraw();

            self.objs = self.objs.iter().map(|o| mv(o, &self.screen)).collect_vec();
        }
    }

    fn create_ran_obj(screen: &UVec2) -> Obj {
        let mut rng = rand::thread_rng();
        let pos = Vector2 {
            x: rng.gen::<f32>() * screen.x as f32,
            y: rng.gen::<f32>() * screen.y as f32,
        };
        let dir = Vector2 {
            x: rng.gen::<f32>() * 50.0 - 25.0,
            y: rng.gen::<f32>() * 50.0 - 25.0,
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
}
