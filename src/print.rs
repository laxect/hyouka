use crate::vv;
use colored::Colorize;
use std::{io, io::Write};

pub fn section(section: &str) {
    // all section should output only when verbose
    if !vv() {
        return;
    }
    println!("{} {}", "::<>".cyan(), section.black());
}

pub fn info(title: String) {
    print!("  {}...", title.bright_black());
    io::stdout().flush().ok();
}

pub fn verbose(title: String) {
    if vv() {
        print!("  {}...", title.bright_black());
        io::stdout().flush().ok();
    }
}

pub trait Check {
    fn check(self) -> Self;
    fn verbose(self) -> Self;
}

impl<A, B> Check for Result<A, B> {
    fn check(self) -> Self {
        if self.is_ok() {
            println!("{}", "OK".green())
        } else {
            println!("{}", "Failed".red())
        }
        self
    }

    fn verbose(self) -> Self {
        if vv() {
            if self.is_ok() {
                println!("{}", "OK".green());
            } else {
                println!("{}", "Failed".red());
            }
        }
        self
    }
}
