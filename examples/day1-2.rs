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
    gnomes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_three: i32 = gnomes.iter().take(3).sum();
    println!("Calories from top three gnomes: {}", top_three);
    Ok(())
}
