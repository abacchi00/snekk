use macroquad::prelude::*;
use std::collections::VecDeque;

// Constants
const BOARD_SIZE: usize = 20;
const CELL_SIZE: f32 = 30.0;
const TICK_RATE: f64 = 0.1; // 100ms

// Types, Structs and Implementations 
type BoardState = [[u8; BOARD_SIZE]; BOARD_SIZE];

type SnekkBody = VecDeque<Pos>;

#[derive(Clone, Copy, PartialEq)]
struct Pos {
  x: usize,
  y: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn from_keycode(keycode: Option<KeyCode>) -> Option<Self> {
    match keycode {
      Some(KeyCode::Up) => Some(Direction::Up),
      Some(KeyCode::Down) => Some(Direction::Down),
      Some(KeyCode::Left) => Some(Direction::Left),
      Some(KeyCode::Right) => Some(Direction::Right),
      _ => None
    } 
  }

  fn opposite(self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

struct Snekk {
  body: SnekkBody,
  direction: Direction,
  pos: Pos,
}

impl Snekk {
  const INITIAL_POS: [Pos; 3] = [
    Pos { x: 3, y: 1 },
    Pos { x: 2, y: 1 },
    Pos { x: 1, y: 1 },
  ];

  fn new() -> Self {
    return Self {
      body: VecDeque::from(Self::INITIAL_POS),
      direction: Direction::Right,
      pos: Self::INITIAL_POS[0],
    }
  }

  fn change_direction(&mut self, maybe_new_direction: Option<Direction>) {
    if let Some(new_direction) = maybe_new_direction {
      if new_direction == self.direction.opposite() { return; }

      self.direction = new_direction;
    }
  }

  fn advance(&mut self, apple: &mut Apple) {
    match self.direction {
      Direction::Up => self.pos.y = self.pos.y.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Down => self.pos.y = (self.pos.y + 1) % BOARD_SIZE,
      Direction::Left => self.pos.x = self.pos.x.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Right => self.pos.x = (self.pos.x + 1) % BOARD_SIZE,
    }

    self.body.push_front(self.pos);

    // TODO: remove apple regeneration responsability from here
    if self.pos != apple.pos {
      self.body.pop_back();
    } else {
      apple.goto_valid_position(&self.body);
    }
  }
}

struct Apple {
  pos: Pos,
}

impl Apple {
  fn new(prohibited_positions: &VecDeque<Pos>) -> Self {
    return Self {
      pos: Self::generate_valid_position(prohibited_positions),
    }
  }

  fn goto_valid_position(&mut self, prohibited_positions: &VecDeque<Pos>) {
    self.pos = Self::generate_valid_position(prohibited_positions);
  }

  fn generate_valid_position(prohibited_positions: &VecDeque<Pos>) -> Pos {
    loop {
      let candidate = Pos {
        x: rand::gen_range(0, BOARD_SIZE),
        y: rand::gen_range(0, BOARD_SIZE),
      };
  
      if !prohibited_positions.iter().any(|pos| pos.x == candidate.x && pos.y == candidate.y) {
        break candidate;
      }
    }
  }
}

// State Updaters
fn update_board_state(board_state: &mut BoardState, snekk_body: &mut SnekkBody, apple: &Apple) {
  for row in board_state.iter_mut() {
    row.fill(0);
  }

  board_state[apple.pos.x][apple.pos.y] = 2;

  for segment_pos in snekk_body {
    board_state[segment_pos.x][segment_pos.y] = 1;
  }
}

// Render Functions
fn render_game(board_state: &BoardState, snekk_body_len: usize) {
  clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

  let offset_x = (screen_width() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;
  let offset_y = (screen_height() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;

  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      let cell = board_state[x][y];
      
      let px = offset_x + (x as f32 * CELL_SIZE);
      let py = offset_y + (y as f32 * CELL_SIZE);

      if cell == 0 {
        draw_circle(px + CELL_SIZE / 2.0, py + CELL_SIZE / 2.0, 2.0, DARKGRAY);
      } else if cell == 1 {
        draw_rectangle(px + 1.0, py + 1.0, CELL_SIZE - 2.0, CELL_SIZE - 2.0, GREEN);
      } else if cell == 2 {
        draw_circle(px + CELL_SIZE / 2.0, py + CELL_SIZE / 2.0, CELL_SIZE * 0.4, RED);
      }
    }
  }

  draw_text(
    &format!("Snake size: {}", snekk_body_len),
    20.0,
    30.0,
    30.0,
    WHITE,
  );
}

// Core Game Loop
#[macroquad::main("Snekk")]
async fn main() {
  let mut board_state: BoardState = [[0; BOARD_SIZE]; BOARD_SIZE];
  let mut snekk = Snekk::new();
  let mut apple = Apple::new(&snekk.body);
  let mut last_tick = get_time();
  
  // Seed the random number generator so apples are random on every launch
  rand::srand(macroquad::miniquad::date::now() as u64);

  loop {
    snekk.change_direction(Direction::from_keycode(get_last_key_pressed()));

    if get_time() - last_tick >= TICK_RATE {
      snekk.advance(&mut apple);
      update_board_state(&mut board_state, &mut snekk.body, &apple);
      
      last_tick = get_time();
    }

    render_game(&board_state, snekk.body.len());

    next_frame().await;
  }
}
