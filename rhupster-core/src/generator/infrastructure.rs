use crate::config::ProjectConfig;
use anyhow::Result;
use std::path::Path;
use super::utils::{render_file_from_template, TemplateSource};

pub async fn generate(config: &ProjectConfig, template_root: TemplateSource<'_>, output_dir: &Path) -> Result<()> {
     if !config.devops.docker_compose {
        return Ok(());
    }
    println!("Generating Infrastructure...");

    let template_path = "infrastructure/docker-compose.yml.tera";
    
    if template_root.has_file(template_path) {
        render_file_from_template(config, template_root, template_path, output_dir.join("docker-compose.yml")).await?;
    }

    Ok(())
}
