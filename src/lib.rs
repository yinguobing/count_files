use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// Config struct converts the user inputs into arguments.
pub struct Config {
    pub target_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Target dir provided?
        if args.len() < 2 {
            return Err(
                "target directory not assigned.\nTry running like this:\n   count_files /path/to/dir",
            );
        }
        // Target dir exist?
        let target_path = args[1].to_string();
        if !Path::new(&target_path).exists() {
            return Err("target not found.");
        }
        // Target is a directory?
        if !Path::new(&target_path).is_dir() {
            return Err("target is not a directory.");
        }
        // All is well.
        Ok(Self {
            target_path: target_path,
        })
    }
}

// Count all the files in this function.
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Use a hashmap to record different files count.
    let mut counter: HashMap<String, usize> = HashMap::new();
    println!("Counting files in {}", config.target_path);
    let entries = fs::read_dir(config.target_path.clone())?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            let extension = extension.to_str().unwrap().to_string();
            let count = counter.entry(extension).or_insert(0);
            *count += 1;
        }
    }
    println!("Total files: {:?}", counter);
    Ok(())
}