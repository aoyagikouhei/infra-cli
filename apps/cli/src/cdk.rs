use std::path::PathBuf;

pub fn execute(
    project_code: &str,
    mut path: PathBuf,
) -> anyhow::Result<()> {
    path.push("cdk");
    std::fs::create_dir_all(path.clone())?;

    Ok(())
}

fn execute_cdk() -> anyhow::Result<()> {
    
    Ok(())
}