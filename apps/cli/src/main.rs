use clap::{Parser, Subcommand};

mod cdk;
mod development;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// target dir
    #[arg(short, long)]
    dir: Option<String>,

    /// project code
    #[arg(short, long)]
    project: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// docker-compose
    Development {
        #[arg(short, long)]
        redis: bool,

        #[arg(short, long)]
        postgresql: bool,
    },

    /// CDK
    Cdk,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Development { redis, postgresql } => {
            development::execute(&args.project, &args.dir, redis, postgresql)?
        }
        Command::Cdk => cdk::execute()?,
    }
    Ok(())
}
