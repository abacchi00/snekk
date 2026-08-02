use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::{Instant, Duration};
use std::collections::VecDeque;
use rand;

// Constants
const BOARD_SIZE: usize = 20;
const EMPTY_CELL_STR: &str = " ·";
const SNAKE_CELL_STR: &str = " ■";
const APPLE_CELL_STR: &str = " ✱";
const TICK_RATE: Duration = Duration::from_millis(100);

// Types & Structs
type BoardState = [[u8; BOARD_SIZE]; BOARD_SIZE];
type SnekkBody = VecDeque<SnekkPos>;

#[derive(Clone, Copy, PartialEq)]
struct SnekkPos {
  x: usize,
  y: usize,
}

// Utils
fn clear_terminal() {
  print!("\x1B[2J\x1B[1;1H");
}

// State Updaters

fn update_current_dir(current_dir: &mut Keycode, keys: &[Keycode]) {
  if keys.contains(&Keycode::Up) && *current_dir != Keycode::Down    { *current_dir = Keycode::Up };
  if keys.contains(&Keycode::Down) && *current_dir != Keycode::Up    { *current_dir = Keycode::Down };
  if keys.contains(&Keycode::Left) && *current_dir != Keycode::Right { *current_dir = Keycode::Left };
  if keys.contains(&Keycode::Right) && *current_dir != Keycode::Left { *current_dir = Keycode::Right };
}

fn update_snake_pos(snekk_pos: &mut SnekkPos, current_dir: Keycode) {
  let increment: usize = 1;

  if current_dir == Keycode::Up {
    snekk_pos.x = snekk_pos.x.checked_sub(increment).unwrap_or(BOARD_SIZE - increment);
  } else if current_dir == Keycode::Down {
    snekk_pos.x = (snekk_pos.x + increment) % BOARD_SIZE;
  } else if current_dir == Keycode::Right {
    snekk_pos.y = (snekk_pos.y + increment) % BOARD_SIZE;
  } else {
    snekk_pos.y = snekk_pos.y.checked_sub(increment).unwrap_or(BOARD_SIZE - increment);
  }
}

fn update_snake_body(snekk_body: &mut SnekkBody, new_pos: SnekkPos, apple_pos: &mut SnekkPos) {
  let snekk_head_pos = *snekk_body.front().unwrap();

  snekk_body.push_front(new_pos);

  if snekk_head_pos != *apple_pos {
    snekk_body.pop_back();
  } else {
    *apple_pos = generate_apple_pos(&snekk_body);
  }
}

fn update_board_state(board_state: &mut BoardState, snekk_body: &mut SnekkBody, apple_pos: SnekkPos) {
  for row in board_state.iter_mut() {
    row.fill(0);
  }

  board_state[apple_pos.x][apple_pos.y] = 2;

  for segment_pos in snekk_body {
    board_state[segment_pos.x][segment_pos.y] = 1;
  }
}

fn generate_apple_pos(snekk_body: &SnekkBody) -> SnekkPos {
  loop {
    let candidate = SnekkPos {
      x: rand::random_range(0..BOARD_SIZE),
      y: rand::random_range(0..BOARD_SIZE),
    };

    if !snekk_body.iter().any(|pos| pos.x == candidate.x && pos.y == candidate.y) {
      break candidate;
    }
  }
}

// Render Functions
fn render_snekk_pos(snekk_pos: &SnekkPos) {
  println!(" Snekk pos - x: {} y: {}", snekk_pos.x, snekk_pos.y);
}

fn render_board(board_state: &BoardState) {
  board_state.iter().for_each(|line| {
    line.iter().for_each(|cell| {
      let cell_str = 
        if *cell == 0 { EMPTY_CELL_STR }
        else if *cell == 1 { SNAKE_CELL_STR }
        else { APPLE_CELL_STR };
      print!("{}", cell_str);
    });
    println!();
  });
}

fn render_game(snekk_pos: &SnekkPos, board_state: &BoardState) {
  clear_terminal();
  render_snekk_pos(snekk_pos);
  render_board(board_state);
}

// Core Game Loop

fn main() {
  let device_state = DeviceState::new();

  let mut board_state: BoardState = [[0; BOARD_SIZE]; BOARD_SIZE];
  let mut snekk_pos = SnekkPos { x: 1, y: 3 };
  let mut snekk_body: SnekkBody = VecDeque::from([
    SnekkPos { x: 1, y: 3 },
    SnekkPos { x: 1, y: 2 },
    SnekkPos { x: 1, y: 1 },
  ]);
  let mut apple_pos: SnekkPos = generate_apple_pos(&snekk_body);
  let mut last_tick = Instant::now();
  let mut current_dir = Keycode::Right; 
  
  loop {
    update_current_dir(&mut current_dir, &device_state.get_keys());

    if last_tick.elapsed() >= TICK_RATE {
      update_snake_pos(&mut snekk_pos, current_dir);
      update_snake_body(&mut snekk_body, snekk_pos, &mut apple_pos);
      update_board_state(&mut board_state, &mut snekk_body, apple_pos);
      
      render_game(&snekk_pos, &board_state);
      println!("{}", current_dir); // temp

      last_tick = Instant::now();
    }

    // Protects cpu from running loop too fast unnecessarily
    thread::sleep(Duration::from_millis(1));
  }
}
