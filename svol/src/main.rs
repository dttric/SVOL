/* правила компилятора */
#![allow(unused_imports)] // РАЗРАБ
#![allow(unused_variables)] // ДАУН

/* импорты */
use std::thread;
use std::time::Duration;
use console::Term;
use console::Style;
use colorized::*;

/* функции */
fn windows() -> bool
{
  if cfg!(target_os = "windows"){
    return true;
  } else {
    return false;
  }
}

fn ret_error(text: &str)
{
  println!("[ERROR] {}", text.color(Colors::RedBg))
}

fn ret_success(text: &str)
{
  println!("[SUCCESS] {}", text.color(Colors::GreenBg).color(Colors::BlackFg))
}

fn log(text: &str)
{
  println!("[LOG] {}", text.color(Colors::YellowBg).color(Colors::BlackFg))
}

fn main()
{
  /* проверка на винду */
  if windows() == false {
    ret_error("Запустите программу на Windows!");
  } else {
    ret_success("Ошибок не обнаружено...");
    log("Запускаюсь...")
  }
}