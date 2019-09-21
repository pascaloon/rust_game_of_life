// piston crates
extern crate vecmath;
extern crate piston_window;
extern crate find_folder;

// Game of life crate
extern crate game_of_life;

use piston_window::*;
use piston::window::WindowSettings;

use game_of_life::manager;
use game_of_life::manager::Game;
use game_of_life::manager::CellState;

pub struct UserInput {
    left: bool,
    right: bool,
    up: bool,
    down: bool
}

pub struct App {
    user_input: UserInput,
    game: Game,
    dt: f64,
    /// Steps per second
    speed: f64
}
impl App {
    pub fn new() -> App {
        App {
            speed: 5.0,
            dt: 0.0,
            game: manager::load_game_from_file(&String::from("map")).expect("Error while parsing map file."),
            user_input: UserInput {
                left: false,
                right: false,
                up: false,
                down: false
            },
        }
    }

    fn render(&mut self, args: &Event, window: &mut PistonWindow) {
        const SOFTBLACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        window.draw_2d(args, |c: Context, gl: &mut G2d| {
            // Clear the screen.
            clear(SOFTBLACK, gl);
            
            // Render World
            let height = self.game.map.get_height();
            let width = self.game.map.get_width();
            for y in 0..height {
                for x in 0..width {
                    match *self.game.map.get_cell(x, y) {
                        CellState::Alive => {
                            rectangle(
                                [1.0, 0.0, 0.0, 1.0], 
                                [(x as f64) * 10.0, (y as f64) * 10.0, 10.0, 10.0], 
                                c.transform, 
                                gl);
                        },
                        CellState::Dead => { }
                    }
                }
            }

        });
    }

    fn buttons_pressed(&mut self, args: &ButtonArgs) {
        let is_pressed = args.state == ButtonState::Press;
        match args.button {
            Button::Keyboard(k) => match k {
                Key::Space => {
                    if is_pressed {

                    }
                }
                Key::A => self.user_input.left = is_pressed,
                Key::D => self.user_input.right = is_pressed,
                Key::W => self.user_input.up = is_pressed,
                Key::S => self.user_input.down = is_pressed,
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.dt += self.speed * args.dt;
        let steps = self.dt as i32;
        self.dt -= steps as f64;

        if steps > 0 {
            self.game.run(steps as usize);
        }
    }
}


fn main() {
        // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    let (width, height) = (1000.0, 800.0);

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "Game of life",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(b) = e.button_args() {
            app.buttons_pressed(&b);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        app.render(&e, &mut window);

    }
}
