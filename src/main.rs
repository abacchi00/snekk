use macroquad::prelude::*;

mod constants;
mod position;
mod direction;
mod snekk;
mod apple;
mod render;

use constants::{TICK_RATE, BOARD_LEN};
use direction::Direction;
use snekk::Snekk;
use apple::Apple;
use render::render_game;

// Core Game Loop
#[macroquad::main("Snekk")]
async fn main() {
  let mut snekk = Snekk::new();
  let mut apple = Apple::new(&snekk.body);
  let mut last_tick = get_time();
  let mut victory: bool = false;
  
  // Seed the random number generator so apples are random on every launch
  rand::srand(macroquad::miniquad::date::now() as u64);

  loop {
    if let Some(pressed_key_code) = get_last_key_pressed() {
      if (victory || !snekk.alive) && pressed_key_code == KeyCode::Space {
        // Reset the game
        snekk = Snekk::new();
        apple = Apple::new(&snekk.body);
        victory = false;
      }

      if pressed_key_code == KeyCode::Escape {
        break;
      }

      if let Some(new_direction) = Direction::from_keycode(pressed_key_code) {
        snekk.change_direction(new_direction);
      }
    }

    if !victory && snekk.alive && get_time() - last_tick >= TICK_RATE {
      snekk.advance();

      if !snekk.alive {}
      else if snekk.body.len() == BOARD_LEN { victory = true; }
      else if snekk.pos != apple.pos { snekk.shrink(); }
      else { apple.goto_valid_position(&snekk.body); }
      
      last_tick = get_time();
    }

    render_game(&snekk, &apple, victory);

    next_frame().await;
  }
}
