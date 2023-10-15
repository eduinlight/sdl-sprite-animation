use sdl2::image::LoadTexture;
use sdl2::{
  rect::{Point, Rect},
  render::{Texture, TextureCreator, WindowCanvas},
  video::WindowContext,
};

use crate::Entity;

#[derive(Clone, Debug)]
pub enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

#[derive(Clone, Debug)]
pub enum PlayerState {
  Walk(Direction),
  Stop(Direction),
  ScaleUp,
  ScaleDown,
}

pub struct Player<'a> {
  position: Point,
  sprite: Texture<'a>,
  sprite_state: i32,
  sprite_width: u32,
  sprite_height: u32,
  sprite_frame_start: u32,
  sprite_frame_time: u32,
  width: u32,
  height: u32,
  scale_rate: u32,
  state: PlayerState,
  velocity: i32,
}

impl<'a> Player<'a> {
  pub fn new(
    texture_creator: &'a TextureCreator<WindowContext>,
    position: Point,
    sprite_frame_start: u32,
  ) -> Player<'a> {
    let sprite = texture_creator
      .load_texture("assets/sprites/player.png")
      .expect("error loading textuer");

    Player {
      position,
      width: 32,
      height: 32,
      state: PlayerState::Walk(Direction::Up),
      sprite_frame_start,
      sprite_frame_time: 200,
      sprite_state: 0,
      sprite_width: 32,
      sprite_height: 32,
      sprite,
      scale_rate: 5,
      velocity: 2,
    }
  }
}

impl<'a> Entity<PlayerState> for Player<'a> {
  fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.copy(
      &self.sprite,
      Rect::new(
        match self.state.clone() {
          PlayerState::Stop(direction) => direction as i32,
          PlayerState::Walk(direction) => direction as i32,
          _ => 0,
        } * 2
          * self.sprite_width as i32
          + self.sprite_state * self.sprite_width as i32,
        0,
        self.sprite_width,
        self.sprite_height,
      ),
      Rect::new(
        self.position.x,
        self.position.y,
        self.width as u32 * 2,
        self.height as u32 * 2,
      ),
    )?;
    Ok(())
  }

  fn update(&mut self, ticks: u32, state: PlayerState) {
    self.state = state.clone();
    let delay = ticks - self.sprite_frame_start;
    if delay >= self.sprite_frame_time {
      self.sprite_state = (self.sprite_state + 1) % self.velocity as i32;
      self.sprite_frame_start = ticks;
    }
    println!("{}", delay);
    match state {
      PlayerState::Walk(direction) => {
        let mut time = 1;
        match self.state {
          PlayerState::Walk(_) => {}
          _ => {
            time = delay;
          }
        }
        match direction {
          Direction::Up => {
            self.position.y = self.position.y - self.velocity * time as i32;
          }
          Direction::Down => {
            self.position.y = self.position.y + self.velocity * time as i32;
          }
          Direction::Right => {
            self.position.x = self.position.x + self.velocity * time as i32;
          }
          Direction::Left => {
            self.position.x = self.position.x - self.velocity * time as i32;
          }
        }
      }
      PlayerState::Stop(_) => {
        self.sprite_state = 0;
      }
      PlayerState::ScaleUp => {
        self.width = self.width + self.scale_rate;
        self.height = self.height + self.scale_rate;
      }
      PlayerState::ScaleDown => {
        self.width = self.width - self.scale_rate;
        self.height = self.height - self.scale_rate;
      }
    }
  }
}
