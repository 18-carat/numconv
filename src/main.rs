#![forbid(unsafe_code)]

mod integer;
mod tests;

use dialoguer::console::Term;
use dialoguer::Input;
use integer::Integer;

fn main() {
    loop {
        let _ = Term::stdout().clear_screen();

        println!("╭─────────────────────╮");
        println!("│ My Number Converter │");
        println!("╰─────────────────────╯");

        let in_fmt = get_in_fmt();
        let out_fmt = get_out_fmt();
        let number = get_number(in_fmt);

        match out_fmt {
            1 => println!("Binary: {}\n", number.to_binary()),
            2 => println!("Decimal: {}\n", number.to_decimal()),
            3 => println!("Hex: {}\n", number.to_hex()),
            _ => panic!("Invalid output format"),
        }

        let input: String = Input::new()
            .with_prompt("Go again? (y/n)")
            .interact_text()
            .unwrap();

        if input.to_lowercase() != "y" {
            break;
        }
    }
}

fn get_in_fmt() -> u8 {
    println!("\n1. Binary  (Unsigned)");
    println!("2. Binary  (Signed)");
    println!("3. Decimal (Unsigned)");
    println!("4. Decimal (Signed)");
    println!("5. Hex     (Unsigned)");
    println!("6. Hex     (Signed)\n");

    let in_fmt_str: String = Input::new()
        .with_prompt("Which input format?")
        .interact_text()
        .unwrap();

    in_fmt_str.parse::<u8>().unwrap()
}

fn get_out_fmt() -> u8 {
    println!("\n1. Binary");
    println!("2. Decimal");
    println!("3. Hex\n");

    let out_fmt_str: String = Input::new()
        .with_prompt("Which output format?")
        .interact_text()
        .unwrap();

    out_fmt_str.parse::<u8>().unwrap()
}

fn get_number(fmt: u8) -> Integer {
    let input: String = Input::new()
        .with_prompt("\nEnter number")
        .interact_text()
        .unwrap();

    println!();

    match fmt {
        1 => Integer::new::<usize>(&input, 2, false),
        2 => Integer::new::<isize>(&input, 2, true),
        3 => Integer::new::<usize>(&input, 10, false),
        4 => Integer::new::<isize>(&input, 10, true),
        5 => Integer::new::<usize>(&input, 16, false),
        6 => Integer::new::<isize>(&input, 16, true),
        _ => panic!("Invalid input format"),
    }
}
