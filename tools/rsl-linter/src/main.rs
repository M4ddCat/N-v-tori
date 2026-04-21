use anyhow::Result;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod linter;
mod rules;

use linter::{Linter, Severity};

#[derive(Parser)]
#[command(name = "rsl-lint", version = "0.1.0", about = "RSL linter")]
struct Cli {
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    #[arg(short, long)]
    fix: bool,

    #[arg(short, long)]
    ignore: Vec<String>,

    #[arg(short, long)]
    recursive: bool,
}

fn read_file_cp866(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    
    if let Ok(text) = String::from_utf8(bytes.clone()) {
        return Ok(text);
    }
    
    let (text, _, _) = encoding_rs::IBM866.decode(&bytes);
    Ok(text.to_string())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut all_diagnostics = Vec::new();
    let mut files = Vec::new();

    for path in &cli.paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() && cli.recursive {
            collect_rsl_files(path, &mut files);
        }
    }

    let linter = Linter::new(cli.ignore.clone());

    for file in files {
        let content = read_file_cp866(&file)?;
        let diagnostics = linter.lint_file(&file, &content, cli.fix);
        all_diagnostics.extend(diagnostics);
    }

    for diag in &all_diagnostics {
    let sev = match diag.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    };
    
    println!("{}:{}:{}: {}: {} [{}]", 
        diag.file.display(), 
        diag.line, 
        diag.column, 
        sev, 
        diag.message,
        diag.rule
    );
}

    if all_diagnostics.iter().any(|d| d.severity == Severity::Error) {
        std::process::exit(1);
    }

    Ok(())
}

fn collect_rsl_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "mac" || ext == "rsl" {
                        files.push(path);
                    }
                }
            } else if path.is_dir() {
                collect_rsl_files(&path, files);
            }
        }
    }
}