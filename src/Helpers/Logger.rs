use colored::*;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use std::fmt::Display;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
}

pub struct Logger;

impl Logger {
    fn StripColors(text: &str) -> String {
        Regex::new(r"\x1b\[[0-9;]*m").unwrap().replace_all(text, "").to_string()
    }

    fn Timestamp() -> String {
        let now = chrono::Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn Prefix(label: &str, symbol: &str, color: Color) -> String {
        format!(
            "{} {}",
            symbol.color(color).bold(),
            label.color(color).bold()
        )
    }

    fn LogInternal(prefix: &str, class_func: &str, message: &str, color: Color) {
        let time = Self::Timestamp();
        let formatted = format!(
            "{} {} {} {}",
            time.dimmed(),
            prefix,
            class_func.cyan().bold(),
            message.bold()
        );

        println!("{}", formatted);

        if let Some(ref mut file) = *LOG_FILE.lock().unwrap() {
            let clean = Self::StripColors(&formatted);
            writeln!(file, "{}", clean).ok();
        }
    }

    // ===== Simple 3-argument log functions =====
    pub fn Debug(class_name: &str, func_name: &str, msg: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        Self::LogInternal(&Self::Prefix("[SERVER]", ">", Color::Cyan), &class_func, msg, Color::Cyan);
    }

    pub fn Log(class_name: &str, func_name: &str, msg: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        Self::LogInternal(&Self::Prefix("[LOG]", ">", Color::BrightGreen), &class_func, msg, Color::BrightGreen);
    }

    pub fn Warn(class_name: &str, func_name: &str, msg: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        Self::LogInternal(&Self::Prefix("[WARNING]", ">", Color::Yellow), &class_func, msg, Color::Yellow);
    }

    pub fn Error(class_name: &str, func_name: &str, msg: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        Self::LogInternal(&Self::Prefix("[ERROR]", "× ", Color::Red), &class_func, msg, Color::Red);
    }

    pub fn Success(class_name: &str, func_name: &str, msg: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        Self::LogInternal(&Self::Prefix("[SUCCESS]", "✓", Color::BrightGreen), &class_func, msg, Color::BrightGreen);
    }

    pub fn Multi(args: &[&str], class_name: &str, func_name: &str) {
        let class_func = format!("[{}::{}]", class_name, func_name);
        let msg = args.join(" ");
        Self::LogInternal(&Self::Prefix("[LOG]", ">", Color::BrightGreen), &class_func, &msg, Color::BrightGreen);
    }

    pub fn Divider() {
        println!("{}", "──────────────────────────────────────────────────────────────".dimmed());
    }
}
