
mod svg_parser;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "input/beispiel.svg";
    let output_path = "output/beispiel.json";

    fs::create_dir_all("output")?;

    let svg = svg_parser::parse_svg(input_path)?;

    svg_parser::save_json(&svg, output_path)?;

    println!("SVG erfolgreich verarbeitet!");
    println!("JSON gespeichert unter: {}", output_path);

    Ok(())
}
