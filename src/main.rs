use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use slowprint::slow_println;
use std::fs;

fn main() {
    let banner =
        fs::read_to_string("resources/banner.txt").expect("Should have been able to read file.");
    let welcome =
        fs::read_to_string("resources/welcome.txt").expect("Should have been able to read file.");
    let delay = std::time::Duration::from_millis(5);

    slow_println(&format!("{}", style(banner).color256(13)), delay);
    print_seperator("*");
    println!("{}", style(welcome).color256(13));

    print_instruction();
}

pub fn print_seperator(seperator: &str) {
    println!("{}", style(seperator.repeat(71)).color256(14))
}

pub fn print_instruction() {
    let exploiting_gender = fs::read_to_string("resources/exploiting_gender.txt")
        .expect("Should have been able to read file.");
    let making_exploits = fs::read_to_string("resources/making_exploits.txt")
        .expect("Should have been able to read file.");
    let examples =
        fs::read_to_string("resources/examples.txt").expect("Should have been able to read file.");

    let options = vec![
        "1. How to Make an Exploit",
        "2. How to Make a Gender Exploit",
        "3. Examples of Gender Exploits",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .interact()
        .unwrap();

    print_seperator("*");

    match selection {
        0 => {
            println!("{}", style(making_exploits).color256(13));
        }
        1 => {
            println!("{}", style(exploiting_gender).color256(13));
        }
        2 => {
            println!("{}", style(examples).color256(13));
        }
        3 => {
            std::process::exit(0);
        }
        _ => {
            println!("This shouldn't happen");
        }
    }

    print_instruction();
}
