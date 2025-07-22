use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(long, short)]
    shout: bool,

    #[arg(long, short)]
    formal: bool,

    #[arg(value_name = "name")]
    name: String,
}

fn main() {
    let args = Args::parse();
    let mut greeting = if args.formal {
        format!("Good day, {}!", args.name)
    } else {
        format!("Hello, {}!", args.name)
    };

    if args.shout {
        greeting = greeting.to_uppercase();
    }

    println!("{}", greeting);
}
