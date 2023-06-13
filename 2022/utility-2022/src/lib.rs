use std::fs;

pub fn is_demo_mode() -> bool {
    dotenv::dotenv().ok();

    let demo_mode = std::env::var("DEMO_MODE").ok();

    match demo_mode {
        Some(val) => val == "1",
        None => {
            eprintln!("DEMO_MODE not set, defaulting to false");
            false
        }
    }
}

const INPUT: &str = "input.txt";
const DEMO_INPUT: &str = "demo-input.txt";

pub fn get_input() -> String {
    let is_demo = is_demo_mode();

    if is_demo {
        println!("Demo mode is enabled");
    }

    let filename = {
        if is_demo {
            DEMO_INPUT
        } else {
            INPUT
        }
    };

    let input =
        fs::read_to_string(filename).expect(format!("{} failed to read file", filename).as_str());

    input
}

#[cfg(test)]
mod tests {
    use super::*;
}
