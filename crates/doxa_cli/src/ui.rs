use std::fmt::Display;

use console::{style, Emoji};

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

pub fn success<S: Display>(msg: S) {
    println!("{} {}", SPARKLE, style(msg).green());
}

pub fn error<S: Display>(msg: S) {
    println!("{} {}", style("ERROR:").red().bold(), style(msg).red());
}

pub fn print_step<A: Display, B: Display, C: Display>(current: A, total: B, msg: C) {
    println!("{} {}", step(current, total), msg);
}

pub fn step<A: Display, B: Display>(current: A, total: B) -> String {
    format!("{}", style(format!("[{}/{}]", current, total)).bold().dim())
}

pub fn keyword<K: Display>(keyword: K) -> String {
    style(keyword).green().bold().to_string()
}
