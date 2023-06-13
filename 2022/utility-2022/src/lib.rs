pub fn is_demo_mode() -> bool {
    dotenv::dotenv().ok();

    let demo_mode = std::env::var("DEMO_MODE").ok();

    match demo_mode {
        Some(val) => val == "1",
        None => {
            println!("DEMO_MODE not set, defaulting to false");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
