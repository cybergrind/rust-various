use anyhow::{Context, Result};
use clap::Parser;
use either::Either;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    lines: bool,
    #[arg(long, short)]
    words: bool,
    #[arg(long, short)]
    chars: bool,
    /// files to process
    #[clap(name = "files")]
    files: Vec<String>,
}

#[derive(Debug, Default)]
struct FileStats {
    name: String,
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
}

#[derive(Debug, Default)]
struct Stats {
    files: Vec<FileStats>,
    total_lines: Option<usize>,
    total_words: Option<usize>,
    total_chars: Option<usize>,
}

fn read_file(name: &str) -> Result<File> {
    let file = File::open(name).with_context(|| format!("Failed to open file: {}", name))?;
    Ok(file)
}

fn calculate_file_stats(name: &str, file: impl BufRead, args: &Args) -> Result<FileStats> {
    let mut stats = FileStats {
        name: name.to_string(),
        ..Default::default()
    };
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    for line in file.lines() {
        let line = line?;
        lines += 1;
        words += line.split_whitespace().count();
        chars += 1; // newline is also a char
        chars += line.chars().count();
    }
    if args.lines {
        stats.lines = Some(lines);
    }
    if args.words {
        stats.words = Some(words);
    }
    if args.chars {
        stats.chars = Some(chars);
    }
    return Ok(stats);
}

fn run_wordcount(args: Args) -> Result<Stats> {
    let all_file_stats: Vec<Result<FileStats>> = args
        .files
        .iter()
        .map(|name| {
            read_file(name)
                .map(|file| io::BufReader::new(file))
                .and_then(|reader| calculate_file_stats(&name, reader, &args))
        })
        .collect();

    let (ok_files, err_files): (Vec<_>, Vec<_>) =
        all_file_stats
            .into_iter()
            .partition_map(|result| match result {
                Ok(stats) => Either::Left(stats),
                Err(e) => Either::Right(e),
            });

    if !err_files.is_empty() {
        for err in err_files {
            eprintln!("Error: {}", err);
        }
        return Err(anyhow::anyhow!("Some files could not be processed."));
    }

    let stats = Stats {
        total_lines: ok_files
            .iter()
            .map(|f| f.lines)
            .fold_options(0, |acc, c| acc + c),
        total_words: ok_files
            .iter()
            .map(|f| f.words)
            .fold_options(0, |acc, c| acc + c),
        total_chars: ok_files
            .iter()
            .map(|f| f.chars)
            .fold_options(0, |acc, c| acc + c),
        files: ok_files,
    };
    Ok(stats)
}

fn print_file_stat(file: &FileStats) {
    if let Some(lines) = file.lines {
        print!("{}", lines);
    }
    if let Some(words) = file.words {
        print!("\t{}", words);
    }
    if let Some(chars) = file.chars {
        print!("\t{}", chars);
    }
    println!("\t{}", file.name);
}

fn print_stats(stats: &Stats) {
    if stats.files.is_empty() {
        println!("No files processed.");
        return;
    }

    for file in &stats.files {
        print_file_stat(file);
    }

    // print total only if there are more than one file
    if stats.files.len() == 1 {
        return
    }

    if let Some(total_lines) = stats.total_lines {
        print!("{}", total_lines)
    }
    if let Some(total_words) = stats.total_words {
        print!("\t{}", total_words);
    }
    if let Some(total_chars) = stats.total_chars {
        print!("\t{}", total_chars);
    }
    println!("\ttotal")
}

fn main() {
    let mut args = Args::parse();
    if args.files.is_empty() {
        eprintln!("No files provided.");
        std::process::exit(1);
    }

    let show_all = !args.lines && !args.words && !args.chars;
    if show_all {
        args.lines = true;
        args.words = true;
        args.chars = true;
    }

    match run_wordcount(args) {
        Ok(stats) => print_stats(&stats),
        Err(e) => {
            eprintln!("\nExit(1)\nError: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_calculations() {
        let input = "line1\nword2 word3\nline4";
        let reader = Cursor::new(input);
        let args = Args {
            lines: true,
            words: true,
            chars: true,
            files: vec![],
        };
        let stats = calculate_file_stats("file1.txt", reader, &args).unwrap();
        assert_eq!(stats.lines, Some(3));
        assert_eq!(stats.words, Some(4));
        assert_eq!(stats.chars, Some(24));
    }

    #[test]
    fn test_selected_calculations() {
        let input = "line1\nword2 word3\nline4";
        let reader = Cursor::new(input);
        let args = Args {
            lines: false,
            words: true,
            chars: true,
            files: vec![],
        };
        let stats = calculate_file_stats("file1.txt", reader, &args).unwrap();
        assert_eq!(stats.lines, None);
        assert_eq!(stats.words, Some(4));
        assert_eq!(stats.chars, Some(24));
    }
}
