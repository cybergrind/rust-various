use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("wordcount run {:?}", args)
}
