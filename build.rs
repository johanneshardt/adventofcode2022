use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdout};
use std::path::Path;

use std::fs;
use std::io::Write;
use ureq;

const YEAR: u16 = 2022;

// sadly we can't display output in a good way: https://github.com/rust-lang/cargo/issues/7037

/// Reads an .env file from the current directory into a HashMap<String, String>
fn read_env_file() -> io::Result<HashMap<String, String>> {
    let file = File::open(Path::new(".env"))?;
    let reader = BufReader::new(file);

    let mut values = HashMap::new();
    for line in reader.lines() {
        let line = line?; // propagate potential error
        let line = line.trim();

        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Find the position of '='
        if let Some(delimiter_pos) = line.find('=') {
            let line_key = line[..delimiter_pos].trim_end(); // key
            let value = line[delimiter_pos + 1..].trim_start();

            // strip quotes if present
            let value = value
                .strip_prefix('"')
                .unwrap_or(value)
                .strip_suffix('"')
                .unwrap_or(value);
            values.insert(line_key.to_owned(), value.to_owned());
        }
    }
    Ok(values)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=.env"); // Rerun if .env changes

    let session_token = match read_env_file() {
        Ok(variables) => match variables.get("AOC_SESSION") {
            Some(token) => token.clone(),
            None => {
                println!(
                    "cargo:warning=.env file exists, but doesn't contain the AOC_SESSION token."
                );
                return Err(format!("No AOC_SESSION token present in the .env file.").into());
            }
        },
        Err(e) => {
            // Error reading the .env file
            println!(
                "cargo:warning=Error reading .env file: {}. Input files will not be downloaded.",
                e
            );
            return Err(format!("Error reading .env file: {}", e).into());
        }
    };

    // find all 'dayXX' directories
    for entry in fs::read_dir("./src")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dirname) = &path.file_name() {
                if let Some(dirname_str) = dirname.to_str() {
                    if dirname_str.starts_with("day") && dirname_str.len() == 5 {
                        if let Ok(day) = dirname_str[3..].parse::<u8>() {
                            check_and_download_input(day, &session_token, &path)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}


fn check_and_download_input(
    day: u8,
    session_token: &str,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let input_path = dir.join("main.input");
    if !input_path.exists() {
        println!(
            "cargo:warning=Input file '{}' not found. Attempting download...",
            input_path.display()
        );

        if session_token.is_empty() {
            println!(
                "cargo:warning=Skipping download for Day {} because AOC_SESSION token is missing or empty.",
                day
            );
            return Ok(());
        }

        let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

        let request = ureq::get(url)
            .header("Cookie", format!("session={session_token}"))
            .call();

        match request {
            Ok(mut response) => {
                let body = response.body_mut().read_to_string()?;
                let mut file = fs::File::create(&input_path)?;
                // puzzle inputs have a trailing newline
                file.write_all(body.trim_end().as_bytes())?;
                println!(
                    "cargo:warning=Successfully downloaded and saved '{}'",
                    input_path.display()
                );
            }
            Err(e) => {
                println!(
                    "cargo:warning=Failed to download input for Day {}: {}",
                    day, e
                );
                return Err(format!("Failed to download input for Day {}: {}", day, e).into());
            }
        }
    }
    Ok(())
}
