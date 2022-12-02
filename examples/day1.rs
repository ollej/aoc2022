use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut max_calories = 0;
    let mut calories = 0;
    let input = fs::read_to_string("examples/input1.txt")?;
    for line in input.lines() {
        if line.is_empty() {
            max_calories = max_calories.max(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>()?;
        }
    }
    println!("Gnome with most calories: {}", max_calories);
    Ok(())
}
