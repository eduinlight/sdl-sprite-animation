use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

mod entity;
mod player;

use entity::*;
use player::*;

// Size of the window
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
// FPS
const FPS: u64 = 64;

fn handle_event_app(event: Event, quit: &mut bool) {
  match event {
    /* close button clicked */
    Event::Quit { .. } => {
      *quit = true;
    }
    Event::KeyDown {
      keycode: Some(Keycode::Escape),
      ..
    } => {
      *quit = true;
    }
    _ => {}
  }
}

fn handle_event_player(event: Event, ticks: u32, entity: &mut dyn Entity<PlayerState>) {
  match event {
    // player interactions
    Event::KeyDown {
      keycode: Some(Keycode::Left),
      ..
    } => {
      entity.update(ticks, PlayerState::Walk(Direction::Left));
    }
    Event::KeyDown {
      keycode: Some(Keycode::Right),
      ..
    } => {
      entity.update(ticks, PlayerState::Walk(Direction::Right));
    }
    Event::KeyDown {
      keycode: Some(Keycode::Up),
      ..
    } => {
      entity.update(ticks, PlayerState::Walk(Direction::Up));
    }
    Event::KeyDown {
      keycode: Some(Keycode::Down),
      ..
    } => {
      entity.update(ticks, PlayerState::Walk(Direction::Down));
    }
    Event::KeyDown {
      keycode: Some(Keycode::KpPlus),
      ..
    } => {
      entity.update(ticks, PlayerState::ScaleUp);
    }
    Event::KeyDown {
      keycode: Some(Keycode::KpMinus),
      ..
    } => {
      entity.update(ticks, PlayerState::ScaleDown);
    }

    Event::KeyUp {
      keycode: Some(Keycode::Left),
      ..
    } => {
      entity.update(ticks, PlayerState::Stop(Direction::Left));
    }
    Event::KeyUp {
      keycode: Some(Keycode::Right),
      ..
    } => {
      entity.update(ticks, PlayerState::Stop(Direction::Right));
    }
    Event::KeyUp {
      keycode: Some(Keycode::Up),
      ..
    } => {
      entity.update(ticks, PlayerState::Stop(Direction::Up));
    }
    Event::KeyUp {
      keycode: Some(Keycode::Down),
      ..
    } => {
      entity.update(ticks, PlayerState::Stop(Direction::Down));
    }
    _ => {}
  }
}

fn render(
  canvas: &mut WindowCanvas,
  color: Color,
  player: &dyn Entity<PlayerState>,
) -> Result<(), String> {
  canvas.set_draw_color(color);
  canvas.clear();

  // draw here
  player.draw(canvas)?;

  canvas.present();

  Ok(())
}

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let timer = sdl_context.timer()?;

  let _image_context = image::init(InitFlag::PNG);

  let window = video_subsystem
    .window("game tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
    .position_centered()
    .build()
    .expect("could not initialize video subsystem");

  let mut canvas = window
    .into_canvas()
    .build()
    .expect("could not make a canvas");

  let texture_creator = canvas.texture_creator();

  let mut event_pump = sdl_context.event_pump()?;
  let mut ticks = timer.ticks();

  let mut quit: bool = false;
  let mut player: Box<dyn Entity<PlayerState>> =
    Box::new(Player::new(&texture_creator, Point::new(0, 0), ticks));

  let background_color = Color::RGB(255, 255, 255);

  'running: loop {
    ticks = timer.ticks();

    // Handle events
    for event in event_pump.poll_iter() {
      handle_event_app(event.clone(), &mut quit);
      handle_event_player(event.clone(), ticks, player.as_mut());
    }

    if quit {
      break 'running;
    }

    // Render
    render(&mut canvas, background_color, player.as_mut())?;

    // 60 FPS
    std::thread::sleep(std::time::Duration::from_millis(1000 / FPS));
  }

  Ok(())
}
