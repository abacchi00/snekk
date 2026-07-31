use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::{Instant, Duration};

const BOARD_SIZE: usize = 20;
const EMPTY_CELL_STR: &str = " ◯";
const SNAKE_CELL_STR: &str = " ■";
const TICK_RATE: Duration = Duration::from_millis(150);

type BoardState = [[u8; BOARD_SIZE]; BOARD_SIZE];
type SnekkSize = usize;

#[derive(Clone, Copy)]
struct SnekkPos {
  x: usize,
  y: usize,
}

// Utils

fn clear_terminal() {
  // \x1B[2J clears the screen
  // \x1B[1;1H moves the cursor to row 1, column 1
  print!("\x1B[2J\x1B[1;1H");
}

// State Updaters

fn update_current_dir (current_dir: Keycode, keys: Vec<Keycode>) -> Keycode {
  let mut new_dir = current_dir;

  if keys.contains(&Keycode::Up)    { new_dir = Keycode::Up };
  if keys.contains(&Keycode::Down)  { new_dir = Keycode::Down };
  if keys.contains(&Keycode::Left)  { new_dir = Keycode::Left };
  if keys.contains(&Keycode::Right) { new_dir = Keycode::Right };

  return new_dir;
}

fn update_snake_pos (snekk_pos: SnekkPos) -> SnekkPos {
  let increment: usize = 1;

  return SnekkPos {
    y: snekk_pos.y,
    x: (snekk_pos.x + increment) % BOARD_SIZE,
  };
} 

fn update_board_state(
  snekk_size: SnekkSize,
  snekk_pos: SnekkPos,
  mut board_state: BoardState
) -> BoardState {
  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      board_state[x][y] = 0;
    }  
  }

  for i in 0..snekk_size {
    let x: usize = snekk_pos.y;
    let y: usize =
      if snekk_pos.x < i{ snekk_pos.x + BOARD_SIZE - i }
      else { snekk_pos.x - i };

    board_state[x][y] = 1;
  }

  return board_state;
}

// Render Functions

fn render_snekk_pos(snekk_pos: SnekkPos) {
  println!(" Snekk pos - x: {} y: {}", snekk_pos.x, snekk_pos.y);
}

fn render_board(board_state: BoardState) {
  board_state.iter().for_each(|line| {
    line.iter().for_each(|cell| {
      let cell_str =
        if *cell == 0 { EMPTY_CELL_STR }
        else { SNAKE_CELL_STR };

      print!("{}", cell_str);
    });

    println!();
  });
}

fn render_game(snekk_pos: SnekkPos, board_state: BoardState) {
  clear_terminal();
  render_snekk_pos(snekk_pos);
  render_board(board_state);
}

// Core Game Loop

fn main() {
  let device_state = DeviceState::new();

  let mut board_state: BoardState = [[0; BOARD_SIZE]; BOARD_SIZE];
  let mut snekk_pos = SnekkPos { x: 1, y: 3 };
  let snekk_size: usize = 3; // for now not mut

  let mut last_tick = Instant::now();

  let mut current_dir = Keycode::Right; 
  
  loop {
    current_dir = update_current_dir(current_dir, device_state.get_keys());

    if last_tick.elapsed() >= TICK_RATE {
      snekk_pos = update_snake_pos(snekk_pos);
      board_state = update_board_state(snekk_size, snekk_pos, board_state);
      
      render_game(snekk_pos, board_state);
      println!("{}", current_dir); // temp

      last_tick = Instant::now();
    }

    // Protects cpu from running loop too fast unnecessarily
    thread::sleep(Duration::from_millis(1));
  }
  
}
