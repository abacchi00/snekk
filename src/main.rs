use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::{Instant, Duration};
use std::collections::VecDeque;

// Constants

const BOARD_SIZE: usize = 20;
const EMPTY_CELL_STR: &str = " ◯";
const SNAKE_CELL_STR: &str = " ■";
const TICK_RATE: Duration = Duration::from_millis(100);

// Types & Structs

type BoardState = [[u8; BOARD_SIZE]; BOARD_SIZE];
type SnekkBody = VecDeque<SnekkPos>;

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

fn update_current_dir(current_dir: Keycode, keys: Vec<Keycode>) -> Keycode {
  let mut new_dir = current_dir;

  if keys.contains(&Keycode::Up) && current_dir != Keycode::Down    { new_dir = Keycode::Up };
  if keys.contains(&Keycode::Down) && current_dir != Keycode::Up    { new_dir = Keycode::Down };
  if keys.contains(&Keycode::Left) && current_dir != Keycode::Right { new_dir = Keycode::Left };
  if keys.contains(&Keycode::Right) && current_dir != Keycode::Left { new_dir = Keycode::Right };

  return new_dir;
}

fn update_snake_pos(snekk_pos: SnekkPos, current_dir: Keycode) -> SnekkPos {
  let increment: usize = 1;

  if current_dir == Keycode::Up {
    return SnekkPos {
      x: snekk_pos.x.checked_sub(increment).unwrap_or(BOARD_SIZE - increment),
      y: snekk_pos.y,
    };
  } else if current_dir == Keycode::Down {
    return SnekkPos {
      x: (snekk_pos.x + increment) % BOARD_SIZE,
      y: snekk_pos.y,
    };
  } else if current_dir == Keycode::Right {
    return SnekkPos {
      x: snekk_pos.x,
      y: (snekk_pos.y + increment) % BOARD_SIZE,
    };
  } else {
    return SnekkPos {
      x: snekk_pos.x,
      y: snekk_pos.y.checked_sub(increment).unwrap_or(BOARD_SIZE - increment),
    };
  }
}

fn update_snake_body(mut snekk_body: SnekkBody, new_pos: SnekkPos) -> SnekkBody {
  snekk_body.pop_back();
  snekk_body.push_front(new_pos);

  return snekk_body;
}

fn update_board_state(
  snekk_body: SnekkBody,
  mut board_state: BoardState
) -> BoardState {
  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      board_state[x][y] = 0;
    }  
  }

  for segment in &snekk_body {
    board_state[segment.x][segment.y] = 1;
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
  let mut snekk_body: SnekkBody = VecDeque::from([
    SnekkPos { x: 1, y: 3 },
    SnekkPos { x: 1, y: 2 },
    SnekkPos { x: 1, y: 1 },
  ]);
  let mut last_tick = Instant::now();
  let mut current_dir = Keycode::Right; 
  
  loop {
    current_dir = update_current_dir(current_dir, device_state.get_keys());

    if last_tick.elapsed() >= TICK_RATE {
      snekk_pos = update_snake_pos(snekk_pos, current_dir);
      snekk_body = update_snake_body(snekk_body, snekk_pos);
      board_state = update_board_state(snekk_body.clone(), board_state);
      
      render_game(snekk_pos, board_state);
      println!("{}", current_dir); // temp

      last_tick = Instant::now();
    }

    // Protects cpu from running loop too fast unnecessarily
    thread::sleep(Duration::from_millis(1));
  }
  
}
