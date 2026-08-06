use macroquad::prelude::*;

use crate::constants::{CELL_SIZE, BOARD_SIZE};
use crate::Snekk;
use crate::Apple;

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

fn render_victory_overlay() {
  render_opaque_overlay();
  render_centered_text("You won!", 48.0, -40.0);
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

pub fn render_game(snekk: &Snekk, apple: &Apple, victory: bool) {
  clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

  let offset_x = (screen_width() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;
  let offset_y = (screen_height() - (BOARD_SIZE as f32 * CELL_SIZE)) / 2.0;

  render_board(offset_x, offset_y);

  render_snake(snekk, offset_x, offset_y);

  if !victory { render_apple(apple, offset_x, offset_y); }

  draw_text(
    &format!("Snake size: {}", snekk.body.len()),
    20.0,
    30.0,
    30.0,
    WHITE,
  );

  if !snekk.alive { render_game_over_overlay(); }
  else if victory { render_victory_overlay(); }
}
