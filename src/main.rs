use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use regex::RegexBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    /// Il pattern (Regex) da cercare
    pattern: String,
    /// Il file o la cartella in cui cercare
    path: std::path::PathBuf,
    /// Ignora la differenza tra maiuscole e minuscole
    #[arg(short, long)]
    ignore_case: bool,
}

fn search_in_file(re: &regex::Regex, path: &Path) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Impossibile aprire {:?}", path))?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let content = line?;
        
        // La magia della Regex succede qui
        if re.is_match(&content) {
            let line_num = (index + 1).to_string().dimmed();
            let file_name = path.display().to_string().cyan().bold();
            
            // Coloriamo i match trovati dalla regex
            let highlighted = re.replace_all(&content, |caps: &regex::Captures| {
                caps[0].red().bold().to_string()
            });
            
            println!("{}:{} {}", file_name, line_num, highlighted);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Costruiamo la Regex una volta sola all'inizio
    let re = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| format!("Regex non valida: {}", args.pattern))?;

    if args.path.is_dir() {
        for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let _ = search_in_file(&re, entry.path());
            }
        }
    } else {
        search_in_file(&re, &args.path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_find_a_match() {
        let pattern = "rust";
        let content = "Linguaggio rust\nè fantastico";
        let re = regex::Regex::new(pattern).unwrap();
        
        // Verifichiamo se la regex trova il pattern
        assert!(re.is_match(content));
    }

    #[test]
    fn test_no_match() {
        let pattern = "java";
        let content = "Linguaggio rust\nè fantastico";
        let re = regex::Regex::new(pattern).unwrap();
        
        assert!(!re.is_match(content));
    }
}
