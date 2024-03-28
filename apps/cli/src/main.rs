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

fn execute_development(project_code: &str, dir: &Option<String>, redis: bool, postgresql: bool) {
    let mut path = PathBuf::new();
    if let Some(dir) = dir {
        path.push(dir);
    }
    path.push("development");
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut volumes = vec![
        json!({"v": format!("{}_cargo_cache", project_code)}),
        json!({"v": format!("{}_target_cache", project_code)}),
    ];
    let mut depends = vec![];

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("template", "./templates/development/docker-compose.hbs")
        .unwrap();
    handlebars
        .register_template_file("docker", "./templates/development/Dockerfile.hbs")
        .unwrap();
    if redis {
        handlebars.register_template_file("redis_template", "./templates/development/redis.hbs").unwrap();
        volumes.push(json!({"v": format!("{}_redis_data", project_code)}));
        depends.push(json!({"v": "redis"}));
    }
    if postgresql {
        handlebars.register_template_file("postgresql_template", "./templates/development/postgresql.hbs").unwrap();
        volumes.push(json!({"v": format!("{}_postgresql_data", project_code)}));
        depends.push(json!({"v": "postgresql"}));
    }
    let mut path_docker_compose_yaml = path.clone();
    path_docker_compose_yaml.push("docker-compose.yaml");
    let mut output_file = File::create(path_docker_compose_yaml).unwrap();
    let data = json!({
        "project_code": project_code,
        "redis": redis,
        "postgresql": postgresql,
        "volumes": volumes,
        "volume_flag": !volumes.is_empty(),
        "depends": depends,
        "depend_flag": !depends.is_empty(),
    });
    handlebars.render_to_write("template", &data, &mut output_file).unwrap();

    let mut path_dockerfile = path.clone();
    path_dockerfile.push("Dockerfile.api");
    let mut output_file = File::create(path_dockerfile).unwrap();
    handlebars.render_to_write("docker", &data, &mut output_file).unwrap();
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Development{redis, postgresql} => execute_development(&args.project, &args.dir, redis, postgresql),
        Command::Cdk => println!("cdk"),
    }
}