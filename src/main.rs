use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, BufRead};
use std::thread;
use regex::Regex;
use colored::*;

fn search_file(file_path: &Path, regex: &Regex) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if regex.is_match(&line) {
            println!(
                "{}:{} {}",
                file_path.display(),
                line_number + 1,
                line.trim().yellow()
            );
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file1> [file2] [file3] ...", args[0]);
        return Ok(());
    }

    let pattern = &args[1];
    let regex = Regex::new(pattern)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", e)))?;

    let files: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    let mut handles = vec![];

    for file in files {
        let regex_clone = regex.clone();
        let file = file.to_string();

        let handle = thread::spawn(move || {
            if let Err(err) = search_file(Path::new(&file), &regex_clone) {
                eprintln!("Error in {}: {:?}", &file, err);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
    }

    Ok(())
}
