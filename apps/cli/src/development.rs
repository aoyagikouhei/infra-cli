use std::{fs::File, path::PathBuf};

use handlebars::Handlebars;
use serde_json::json;

pub fn execute(
    project_code: &str,
    dir: &Option<String>,
    redis: bool,
    postgresql: bool,
) -> anyhow::Result<()> {
    let mut path = PathBuf::new();
    if let Some(dir) = dir {
        path.push(dir);
    }
    path.push("development");
    std::fs::create_dir_all(path.clone())?;

    let mut volumes = vec![
        json!({"v": format!("{}_cargo_cache", project_code)}),
        json!({"v": format!("{}_target_cache", project_code)}),
    ];
    let mut depends = vec![];

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("template", "./templates/development/docker-compose.hbs")?;
    handlebars.register_template_file("docker", "./templates/development/Dockerfile.hbs")?;
    if redis {
        handlebars.register_template_file("redis_template", "./templates/development/redis.hbs")?;
        volumes.push(json!({"v": format!("{}_redis_data", project_code)}));
        depends.push(json!({"v": "redis"}));
    }
    if postgresql {
        handlebars.register_template_file(
            "postgresql_template",
            "./templates/development/postgresql.hbs",
        )?;
        volumes.push(json!({"v": format!("{}_postgresql_data", project_code)}));
        depends.push(json!({"v": "postgresql"}));
    }
    let mut path_docker_compose_yaml = path.clone();
    path_docker_compose_yaml.push("docker-compose.yaml");
    let mut output_file = File::create(path_docker_compose_yaml)?;
    let data = json!({
        "project_code": project_code,
        "redis": redis,
        "postgresql": postgresql,
        "volumes": volumes,
        "volume_flag": !volumes.is_empty(),
        "depends": depends,
        "depend_flag": !depends.is_empty(),
    });
    handlebars.render_to_write("template", &data, &mut output_file)?;

    let mut path_dockerfile = path.clone();
    path_dockerfile.push("Dockerfile.app");
    let mut output_file = File::create(path_dockerfile)?;
    handlebars.render_to_write("docker", &data, &mut output_file)?;

    Ok(())
}
