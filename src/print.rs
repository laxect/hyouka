use crate::vv;
use colored::Colorize;
use std::{io, io::Write};

pub fn section(section: &str) {
    // all section should output only when verbose
    if !vv() {
        return;
    }
    println!("{} {}", "::<>".cyan(), section);
}

pub fn verbose<T>(title: T)
where
    T: AsRef<str>,
{
    if vv() {
        print!("  {}...", title.as_ref().bright_black());
        io::stdout().flush().ok();
    }
}

pub fn vline<T>(line: T)
where
    T: AsRef<str>,
{
    if vv() {
        println!("  {}", line.as_ref());
    }
}

pub trait Check {
    fn verbose(self) -> Self;
    fn print_err(self) -> Self;
}

impl<A, E> Check for Result<A, E>
where
    E: std::fmt::Display,
{
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

    fn print_err(self) -> Self {
        match self {
            Ok(_) => {}
            Err(ref err) => {
                println!("{}: {}", "Error".red(), err);
            }
        }
        self
    }
}
