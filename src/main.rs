use std::thread;
use std::time::Duration;

const BOARD_SIZE: usize = 20;

type BoardState = [[u8; BOARD_SIZE]; BOARD_SIZE];
type SnekkSize = usize;
type SnekkPos = (usize, usize);

// Utils

fn clear_terminal() {
  // \x1B[2J clears the screen
  // \x1B[1;1H moves the cursor to row 1, column 1
  print!("\x1B[2J\x1B[1;1H");
}

// State Updaters

fn update_snake_pos (mut snekk_pos: SnekkPos) -> SnekkPos {
  let increment: usize = 1;

  return (snekk_pos.0, (snekk_pos.1 + increment) % BOARD_SIZE);
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
    let x: usize = snekk_pos.0;
    let y: usize =
      if snekk_pos.1 < i{ snekk_pos.1 + BOARD_SIZE - i }
      else { snekk_pos.1 - i };

    board_state[x][y] = 1;
  }

  return board_state;
}

// Render Functions

fn render_snekk_pos(snekk_pos: SnekkPos) {
  println!(" Snekk pos - x: {} y: {}", snekk_pos.0, snekk_pos.1);
}

fn render_board(board_state: BoardState) {
  board_state.iter().for_each(|line| {
    line.iter().for_each(|cell| {
      print!("{}", if *cell == 0 { " ◯" } else { " ■" })
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
  let mut board_state: BoardState = [[0; BOARD_SIZE]; BOARD_SIZE];
  let mut snekk_pos: (usize, usize) = (1, 3);
  let snekk_size: usize = 3; // for now not mut
  
  loop {
    snekk_pos = update_snake_pos(snekk_pos);
    board_state = update_board_state(snekk_size, snekk_pos, board_state);
    
    thread::sleep(Duration::from_millis(150));

    render_game(snekk_pos, board_state);
  }
}
