/* правила компилятора */
#![allow(unused_imports)] // РАЗРАБ
#![allow(unused_variables)] // ДАУН

/* импорты */
use std::thread;
use std::time::Duration;
use console::Term;
use console::Style;
use colorized::*;
use std::io::Write;
use std::io::Read;
use std::env;
use std::format as f;
mod discord;
mod minecraft;

/* функции */
fn windows() -> bool{ return if cfg!(target_os = "windows") { true } else { false }; }
mod format;

fn main()
{
  /* переменные */
  let args: Vec<String> = env::args().collect();
  /* проверка на винду */
  if windows() == false {
    format::fatal("Запустите программу на Windows!")
  } else {
    format::success("Ошибок запуска не обнаружено...");
  }
  /* загрузка ядра */
  format::log("Запускаю модули...");
  format::log("Запуск модуля аргументов...");
  /* модуль аргументов */
  for i in 0..args.len() { println!("[ARGS] {}", f!("Аргумент №{} - {}", i, &args[i]).color(Colors::YellowBg).color(Colors::BlackFg)) }
  if args.len() < 4 {
    format::error("Аргументы не верны! Команда: svol.exe ely.by-login=string ely.by-password=string X.XX.X=version")
  }
  /* discord модуль */
  discord::init();
}