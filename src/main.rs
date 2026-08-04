use macroquad::prelude::*;
use std::collections::VecDeque;

// Constants
const BOARD_SIZE: usize = 20;
const CELL_SIZE: f32 = 30.0;
const TICK_RATE: f64 = 0.1; // 100ms

// Types & Structs
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

// State Updaters
fn update_current_dir(current_dir: &mut Direction) {
  if is_key_pressed(KeyCode::Up) && *current_dir != Direction::Down {
    *current_dir = Direction::Up;
  } else if is_key_pressed(KeyCode::Down) && *current_dir != Direction::Up {
    *current_dir = Direction::Down;
  } else if is_key_pressed(KeyCode::Left) && *current_dir != Direction::Right {
    *current_dir = Direction::Left;
  } else if is_key_pressed(KeyCode::Right) && *current_dir != Direction::Left {
    *current_dir = Direction::Right;
  }
}

fn update_snake_pos(snekk_pos: &mut Pos, current_dir: Direction) {
  match current_dir {
    Direction::Up => snekk_pos.y = snekk_pos.y.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
    Direction::Down => snekk_pos.y = (snekk_pos.y + 1) % BOARD_SIZE,
    Direction::Left => snekk_pos.x = snekk_pos.x.checked_sub(1).unwrap_or(BOARD_SIZE - 1),
    Direction::Right => snekk_pos.x = (snekk_pos.x + 1) % BOARD_SIZE,
  }
}

fn update_snake_body(snekk_body: &mut SnekkBody, new_pos: Pos, apple_pos: &mut Pos) {
  snekk_body.push_front(new_pos);

  if new_pos != *apple_pos {
    snekk_body.pop_back();
  } else {
    *apple_pos = generate_apple_pos(&snekk_body);
  }
}

fn update_board_state(board_state: &mut BoardState, snekk_body: &mut SnekkBody, apple_pos: Pos) {
  for row in board_state.iter_mut() {
    row.fill(0);
  }

  board_state[apple_pos.x][apple_pos.y] = 2;

  for segment_pos in snekk_body {
    board_state[segment_pos.x][segment_pos.y] = 1;
  }
}

fn generate_apple_pos(snekk_body: &SnekkBody) -> Pos {
  loop {
    let candidate = Pos {
      x: rand::gen_range(0, BOARD_SIZE),
      y: rand::gen_range(0, BOARD_SIZE),
    };

    if !snekk_body.iter().any(|pos| pos.x == candidate.x && pos.y == candidate.y) {
      break candidate;
    }
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
  let mut snekk_pos = Pos { x: 3, y: 1 };
  let mut snekk_body: SnekkBody = VecDeque::from([
    Pos { x: 3, y: 1 },
    Pos { x: 2, y: 1 },
    Pos { x: 1, y: 1 },
  ]);
  let mut apple_pos: Pos = generate_apple_pos(&snekk_body);
  let mut last_tick = get_time();
  let mut current_dir = Direction::Right; 
  
  // Seed the random number generator so apples are random on every launch
  rand::srand(macroquad::miniquad::date::now() as u64);

  loop {
    update_current_dir(&mut current_dir);

    if get_time() - last_tick >= TICK_RATE {
      update_snake_pos(&mut snekk_pos, current_dir);
      update_snake_body(&mut snekk_body, snekk_pos, &mut apple_pos);
      update_board_state(&mut board_state, &mut snekk_body, apple_pos);
      
      last_tick = get_time();
    }

    render_game(&board_state, snekk_body.len());

    next_frame().await;
  }
}
