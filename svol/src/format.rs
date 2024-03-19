#![allow(dead_code)]
use colorized::*;
pub fn error(text: &str) { println!("[ERROR] {}", text.color(Colors::RedBg)); }
pub fn success(text: &str) { println!("[SUCCESS] {}", text.color(Colors::GreenBg).color(Colors::BlackFg)); }
pub fn fatal(text: &str) { panic!("[FATAL] {}", text.color(Colors::RedBg)); }
pub fn log(text: &str) { println!("[LOG] {}", text.color(Colors::YellowBg).color(Colors::BlackFg)); }
pub fn warn(text: &str) { println!("[WARN] {}", text.color(Colors::YellowBg).color(Colors::BlackFg)); }