use std::io::Write;
use console::Term;

#[cfg(debug_assertions)]
pub fn read_char() -> char {
    if Term::stdout().features().family() == console::TermFamily::File {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input.chars().next().unwrap_or('\n'),
            Err(_) => '\n'
        }
    } else {
        Term::stdout().read_char().unwrap_or(char::default())
    }
}
#[cfg(not(debug_assertions))]
pub fn read_char() -> char {
    Term::stdout().read_char().unwrap_or(char::default())
}

#[cfg(debug_assertions)]
pub fn read_line(initial: &str) -> String {
    print_flushed(initial);
    if Term::stdout().features().family() == console::TermFamily::File {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => String::new()
        }
    } else {
        Term::stdout().read_line().unwrap_or_default()
    }
}
#[cfg(not(debug_assertions))]
pub fn read_line(initial: &str) -> String {
    print_flushed(initial);
    Term::stdout().read_line().unwrap_or_default()
}

pub fn print_flushed(string: impl std::fmt::Display) {
    print!("{}", string);
    std::io::stdout().flush().unwrap_or(());
}

#[cfg(debug_assertions)]
pub fn clear_screen() {
    if Term::stdout().features().family() != console::TermFamily::File {
        Term::stdout().clear_screen().unwrap_or_default();
    }
}
#[cfg(not(debug_assertions))]
pub fn clear_screen() {
    Term::stdout().clear_screen().unwrap_or_default();
}