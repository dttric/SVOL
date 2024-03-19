/* правила компилятора */
#![allow(unused_imports)] // РАЗРАБ
#![allow(unused_variables)] // ДАУН

/* импорты */
use std::thread;
use std::time;
use console::Term;
use console::Style;
use colorized::*;
use std::io::Write;
use std::io::Read;
use std::env;
use std::format as f;
mod discord;
mod minecraft;
mod format;

/* функции */
fn windows() -> bool{ return if cfg!(target_os = "windows") { true } else { false }; }


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
    format::fatal("Аргументы не верны! Команда: svol.exe ely.by-login=string ely.by-password=string X.XX.X=version")
  }
  /* discord модуль */
  let discord_thread= thread::spawn(move || loop { { discord::init("Играет в игры", "debug", f!("Игрок: {}", &args[1])); thread::sleep(time::Duration::from_secs(60)); } });
  format::success("Discord модуль загружен!");
  discord_thread.join().unwrap();
}