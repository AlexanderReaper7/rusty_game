extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate structopt;


mod player;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use structopt::StructOpt;
use player::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // self.gl.draw_begin(args.viewport())

        self.gl.draw(args.viewport(), |c, gl| 
        {
            // Clear the screen.
            clear(BLACK, gl);
            piston::Grid.draw(line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G)

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct GameArgs
{
    #[structopt(help = "Number of players")] // TODO: error handling
    player_n: usize,
    #[structopt(help = "Number of squares in the x-axis")]
    size_x: usize,
    #[structopt(help = "Number of squares in the y-axis")]
    size_y: usize,
    #[structopt(help = "Set minimum length required for a line to complete", default_value = "3")]
    min_line_len: usize,
    #[structopt(short, long, help = "whether to randomly generate players or manually select")]
    select_player: bool
}

#[derive(Clone)]
pub struct GridSquare
{
    owner_index: Option<usize>
}

pub struct Game
{
    args: GameArgs,
    players: Vec<Player>,
    grid: Vec<GridSquare>
}

impl Game
{
    pub fn new(args: GameArgs) -> Game
    {
        // Create players
        let mut players : Vec<Player>;
        match args.select_player
        {
            true => {
                players = Player::generate_all_combinations()

            },
            false => {
                
            }
        }

        // Create grid
        let grid : Vec<GridSquare> = vec![GridSquare{owner_index: Option::None}; args.size_x * args.size_y];
        players = Vec::with_capacity(args.player_n);
        // let mut i = 0;
        // let mut color_i = 0;
        // let mut icon_i = 0;
        // loop
        // {
        //     println!("{}:{}", color_i, icon_i);
        //     players.push(Player{
        //         color: PlayerColors::from(color_i),
        //         score: 0,
        //         icon: PlayerIcons::from(icon_i)
        //     });

        //     i = i + 1;
        //     if i < args.player_n 
        //     {
        //         icon_i = (i % PlayerIcons::count() as usize) as u8 ; // y
        //         // color_i = (((i / PlayerIcons::count() as usize) as isize * -(PlayerIcons::count() as isize -1) as isize + i as isize) % PlayerColors::count() as isize) as u8 // x
        //         // color_i = ((i % PlayerIcons::count() as usize) % PlayerColors::count() as usize) as u8 // x
        //         color_i = (i % PlayerColors::count() as usize) as u8 // x
        //     } else { break; }
        // }

        return Game{
            args: args,
            grid: grid,
            players: players
        }
    }

    pub fn draw(&mut self, graphics: &mut GlGraphics) 
    {
        println!("drawd")
    }
}

fn main() 
{
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .decorated(true)
        .samples(4)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        game: Game::new(GameArgs::from_args()),
    };

    // Game loop
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
