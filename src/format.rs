#![allow(dead_code)]
use colorized::*;
use std::process::exit;
pub fn error(text: &str) { println!("[ERROR] {}", text.color(Colors::RedBg)); }
pub fn success(logtype: &str, text: &str) { println!("[{}] {}", logtype, text.color(Colors::GreenBg).color(Colors::BlackFg)); }
pub fn fatal(text: &str) { panic!("[FATAL] {}", text.color(Colors::RedBg)); }
pub fn log(logtype: &str, text: &str) { println!("[{}] {}", logtype, text.color(Colors::YellowBg).color(Colors::BlackFg)); }
pub fn warn(text: &str) { println!("[WARN] {}", text.color(Colors::YellowBg).color(Colors::BlackFg)); }
pub fn logo() -> String
{
    return "─────────────────────────────────────────────────────────────\n─██████████████─██████──██████─██████████████─██████────────\n─██░░░░░░░░░░██─██░░██──██░░██─██░░░░░░░░░░██─██░░██─────────\n─██░░██████████─██░░██──██░░██─██░░██████░░██─██░░██─────────\n─██░░██─────────██░░██──██░░██─██░░██──██░░██─██░░██─────────\n─██░░██████████─██░░██──██░░██─██░░██──██░░██─██░░██─────────\n─██░░░░░░░░░░██─██░░██──██░░██─██░░██──██░░██─██░░██─────────\n─██████████░░██─██░░██──██░░██─██░░██──██░░██─██░░██─────────\n─────────██░░██─██░░░░██░░░░██─██░░██──██░░██─██░░██─────────\n─██████████░░██─████░░░░░░████─██░░██████░░██─██░░██████████─\n─██░░░░░░░░░░██───████░░████───██░░░░░░░░░░██─██░░░░░░░░░░██─\n─██████████████─────██████─────██████████████─██████████████─\n─────────────────────────────────────────────────────────────".to_string()
}