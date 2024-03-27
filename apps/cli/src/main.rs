use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// docker-compose
    Development,

    /// CDK
    Cdk,
}


fn execute_development() {
    std::fs::create_dir_all("infra/development").unwrap();
}

fn main() {
    let path = std::env::current_dir().unwrap();
    println!("starting dir: {}", path.display());
    execute_development();

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    match args.command {
        Command::Development => println!("development"),
        Command::Cdk => println!("cdk"),
    }
}