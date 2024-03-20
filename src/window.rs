extern crate pancurses;
use pancurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input, noecho};

pub fn init()
{
  let window = initscr();
  window.printw("Welcome to SVOL!");
  window.refresh();
  window.keypad(true);
  noecho();
  loop {
      match window.getch() {
          Some(Input::Character(c)) => { window.addch(c); },
          Some(Input::KeyDC) => break,
          Some(input) => { window.addstr(&format!("{:?}", input)); },
          None => ()
      }
  }
  endwin();
}