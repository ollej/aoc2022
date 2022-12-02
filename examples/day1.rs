use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut gnomes = vec![];
    let mut calories = 0;
    let input = fs::read_to_string("examples/input1.txt")?;
    for line in input.lines() {
        if line.is_empty() {
            gnomes.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>()?;
        }
    }
    gnomes.sort();
    gnomes.reverse();
    println!("Gnome with most calories: {}", gnomes.first().unwrap());
    Ok(())
}