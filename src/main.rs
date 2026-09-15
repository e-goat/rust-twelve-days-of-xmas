use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    verses: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = include_str!("../verses.toml");
    let config: Config = toml::from_str(content)?;

    for (i, verse) in config.verses.iter().enumerate() {
        println!("Day {}: {}", i + 1, verse);
    }

    Ok(())
}
