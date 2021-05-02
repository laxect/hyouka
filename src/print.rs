use crate::vv;
use colored::Colorize;

#[macro_export]
macro_rules! line {
    ($($arg:tt)+) => {
        if $crate::vv() {
            println!($($arg)+);
        }
    }
}

#[macro_export]
macro_rules! verbose {
    ($arg:expr) => {
        if $crate::vv() {
            print!("  {}...", colored::Colorize::bright_black($arg));
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    };
    ($($arg:tt)+) => {
        if $crate::vv() {
            print!("  {}...", colored::Colorize::bright_black(format!($($arg)+).as_ref()));
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
}

pub fn section(section: &str) {
    // all section should output only when verbose
    if !vv() {
        return;
    }
    println!("{} {}", "::<>".cyan(), section);
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
