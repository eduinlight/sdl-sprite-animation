use sdl2::render::WindowCanvas;

pub trait Entity<T> {
  fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String>;
  fn update(&mut self, ticks: u32, state: T);
}
