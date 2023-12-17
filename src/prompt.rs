use clap::Parser;

const INSERT_SYMBOL: &str = "$";
const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: i16 = 0;

pub fn display(cmd: Prompt) {
    // let last_return_code = sub_matches.get_one::<u8>("last_return_code").unwrap_or(&0);
    // let keymap = sub_matches.get_one::<&str>("keymap").unwrap_or(&"US");
    // let venv_name = sub_matches.get_one::<&str>("venv").unwrap_or(&"");

    let symbol = match cmd.keymap.as_str() {
        COMMAND_KEYMAP => COMMAND_SYMBOL,
        _ => INSERT_SYMBOL,
    };

    let prompt = match (symbol, cmd.last_return_code) {
        (COMMAND_SYMBOL, NO_ERROR) => format!("%F{{3}}{}%f", COMMAND_SYMBOL),
        (COMMAND_SYMBOL, err) => format!("%F{{9}}{}%f %F{{3}}{}%f", err, COMMAND_SYMBOL),
        (_, NO_ERROR) => format!("%F{{5}}{}%f", INSERT_SYMBOL),
        (_, err) => format!("%F{{9}}{} {}%f", err, INSERT_SYMBOL),
    };

    let venv = match cmd.venv.len() {
        0 => String::from(""),
        _ => format!("%F{{11}}|{}|%f ", cmd.venv),
    };

    print!("{}%F{}%f ", venv, prompt);
}

#[derive(Parser, Debug)]
#[command(name = "prompt", author, version, about, long_about = None)]
pub struct Prompt {
    #[arg(short = 'r')]
    last_return_code: i16,
    #[arg(short = 'k')]
    keymap: String,
    #[arg(long = "venv")]
    venv: String,
}
