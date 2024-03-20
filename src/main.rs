/* правила компилятора */
#![allow(unused_imports)] // РАЗРАБ
#![allow(unused_variables)] // ДАУН

/* импорты */
use std::thread;
use std::time;
use colorized::*;
use std::io::Write;
use std::io::Read;
use std::env;
use std::format as f;
mod discord;
mod minecraft;
mod format;
mod window;
/* функции */
fn windows() -> bool{ return if cfg!(target_os = "windows") { true } else { false }; }


fn main()
{
  /* проверка на винду */
  if windows() == false {
    format::fatal("Запустите программу на Windows!")
  } else {
    format::success("INIT", "Ошибок запуска не обнаружено...");
  }
  /* загрузка ядра */
  format::log("MODULES" ,"Запускаю модули...");
  format::log("MODULES" ,"Запуск модуля аргументов...");
  /* discord модуль */
  let discord_thread= thread::spawn(move || loop { { discord::init("Играет в игры", "debug", "Неизвестные данные о игроке".to_string()); thread::sleep(time::Duration::from_secs(60)); } });
  format::success("DISCORD", "Discord модуль загружен!");
  println!("{}", format::logo());
  /* tui модуль */
  window::init();
  format::success("TUI", "Окно инициализировано!");
  /* сцука ну очееень важная часть прям пизда не трогай и ничего не пускай после нее */
  discord_thread.join().unwrap();
}