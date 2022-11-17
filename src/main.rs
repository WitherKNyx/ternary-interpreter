use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("ERROR: Invalid arguments.\nExpected: ./ternary [input_filename]");
    }
    let input_filename = args[1].clone();
    let ternary_string: String = fs::read_to_string(input_filename).expect("ERROR: Could not read file.");
    ternary_string.split(" ");
    Ok(())
}
