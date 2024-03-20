#![allow(dead_code)]
#![allow(unused_must_use)]
use cli_gui::*;
use std::process::exit;
use console::Term;
pub fn init() {
  let term = Term::stdout();
  let mut window = Window::new(Position::new(1, 0), Size::new(100, 50));
  window.set_to_main();
  window.write(Position::new(0, 0), "Hello".to_string(), Color::new(100, 255, 50));
  window.decorate();
  window.render();
  window.read_line(Position::new(10, 10), "input: ", Color::new(100, 100, 100), true);
  term.clear_screen();
  // window.quit();
}