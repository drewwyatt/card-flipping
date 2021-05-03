mod lib;

use lib::find_solution;
use std::io;

fn main() -> io::Result<()> {
    println!("Finding solution for: {}...", "0100110");
    println!("Found: \"{}\"", find_solution("0100110"));
    Ok(())
}
