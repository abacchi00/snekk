use macroquad::prelude::*;
use std::collections::VecDeque;

// Constants
const BOARD_SIZE: usize = 20;
const CELL_SIZE: f32 = 30.0;
const TICK_RATE: f64 = 0.1; // 100ms

// Types, Structs and Implementations 
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
  fn from_keycode(keycode: KeyCode) -> Option<Self> {
    match keycode {
      KeyCode::Up => Some(Direction::Up),
      KeyCode::Down => Some(Direction::Down),
      KeyCode::Left => Some(Direction::Left),
      KeyCode::Right => Some(Direction::Right),
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
  alive: bool,
  body: VecDeque<Pos>,
  direction: Direction,
  next_direction: Direction,
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
      alive: true,
      body: VecDeque::from(Self::INITIAL_POS),
      direction: Direction::Right,
      next_direction: Direction::Right,
      pos: Self::INITIAL_POS[0],
    }
  }

  fn change_direction(&mut self, new_direction: Direction) {
    if new_direction == self.direction.opposite() { return; }

    self.next_direction = new_direction;
  }

  fn advance(&mut self) {
    self.direction = self.next_direction;

    match self.direction {
      Direction::Up => self.pos.y = self.pos.y.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Down => self.pos.y = (self.pos.y + 1) % BOARD_SIZE,
      Direction::Left => self.pos.x = self.pos.x.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
      Direction::Right => self.pos.x = (self.pos.x + 1) % BOARD_SIZE,
    }

    if self.body.iter().any(|pos| pos.x == self.pos.x && pos.y == self.pos.y) {
      self.alive = false;
    }

    self.body.push_front(self.pos);
  }

  fn shrink(&mut self) {
    self.body.pop_back();
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


// Render Functions
fn render_opaque_overlay() {
  draw_rectangle(
    0.0, 
    0.0, 
    screen_width(), 
    screen_height(), 
    Color::new(0.0, 0.0, 0.0, 0.7) 
  );
}

fn render_centered_text(text: &str, font_size: f32, offset_y: f32) {
  let text_dimensions = get_text_center(text, None, font_size as u16, 1.0, 0.0);
  
  draw_text(
    text,
    (screen_width() / 2.0) - text_dimensions.x,
    (screen_height() / 2.0) - text_dimensions.y + offset_y,
    font_size,
    WHITE,
  );
}

fn render_game_over_overlay() {
  render_opaque_overlay();
  render_centered_text("Game Over!", 48.0, -40.0);
  render_centered_text("Press Esc to exit the game", 24.0, 8.0);
  render_centered_text("Press Space to restart the game", 24.0, 48.0);
}

fn render_board(offset_x: f32, offset_y: f32) {
  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      let px = offset_x + (x as f32 * CELL_SIZE);
      let py = offset_y + (y as f32 * CELL_SIZE);
      draw_circle(px + CELL_SIZE / 2.0, py + CELL_SIZE / 2.0, 2.0, DARKGRAY);
    }
  }
}

fn render_snake(snekk: &Snekk, offset_x: f32, offset_y: f32) {
  for segment in &snekk.body {
    let px = offset_x + (segment.x as f32 * CELL_SIZE);
    let py = offset_y + (segment.y as f32 * CELL_SIZE);
    draw_rectangle(px + 1.0, py + 1.0, CELL_SIZE - 2.0, CELL_SIZE - 2.0, GREEN);
  }
}

fn render_apple(apple: &Apple, offset_x: f32, offset_y: f32) {
  let ax = offset_x + (apple.pos.x as f32 * CELL_SIZE);
  let ay = offset_y + (apple.pos.y as f32 * CELL_SIZE);
  draw_circle(ax + CELL_SIZE / 2.0, ay + CELL_SIZE / 2.0, CELL_SIZE * 0.4, RED);
}

fn render_game(snekk: &Snekk, apple: &Apple) {
  clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

  let offset_x = (screen_width() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;
  let offset_y = (screen_height() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;

  render_board(offset_x, offset_y);

  render_snake(snekk, offset_x, offset_y);

  render_apple(apple, offset_x, offset_y);

  draw_text(
    &format!("Snake size: {}", snekk.body.len()),
    20.0,
    30.0,
    30.0,
    WHITE,
  );

  if !snekk.alive { render_game_over_overlay(); }
}

// Core Game Loop
#[macroquad::main("Snekk")]
async fn main() {
  let mut snekk = Snekk::new();
  let mut apple = Apple::new(&snekk.body);
  let mut last_tick = get_time();
  
  // Seed the random number generator so apples are random on every launch
  rand::srand(macroquad::miniquad::date::now() as u64);

  loop {
    if let Some(pressed_key_code) = get_last_key_pressed() {
      if pressed_key_code == KeyCode::Escape {
        break;
      }

      if !snekk.alive &&pressed_key_code == KeyCode::Space {
        // Reset the game
        snekk = Snekk::new();
        apple = Apple::new(&snekk.body);
      }

      if let Some(new_direction) = Direction::from_keycode(pressed_key_code) {
        snekk.change_direction(new_direction);
      }
    }

    if snekk.alive && get_time() - last_tick >= TICK_RATE {
      snekk.advance();

      if !snekk.alive {}
      else if snekk.pos != apple.pos { snekk.shrink(); }
      else { apple.goto_valid_position(&snekk.body); }
      
      last_tick = get_time();
    }

    render_game(&snekk, &apple);

    next_frame().await;
  }
}
