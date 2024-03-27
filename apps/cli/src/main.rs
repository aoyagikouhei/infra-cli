use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use serde_json::json;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// target dir
    #[arg(short, long)]
    dir: Option<String>,

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

fn execute_development(dir: &Option<String>) {
    let mut path = PathBuf::new();
    if let Some(dir) = dir {
        path.push(dir);
    }
    path.push("development");
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("template", "./templates/development/docker-compose.hbs")
        .unwrap();
    path.push("docker-compose.yaml");
    let mut output_file = File::create(path).unwrap();
    handlebars.render_to_write("template", &json!({}), &mut output_file).unwrap();
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Development => execute_development(&args.dir),
        Command::Cdk => println!("cdk"),
    }
}