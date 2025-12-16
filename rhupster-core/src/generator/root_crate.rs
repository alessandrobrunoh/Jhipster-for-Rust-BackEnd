use super::utils::{render_file_from_template, TemplateSource};
use crate::config::{AIAgent, ProjectConfig};
use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub async fn generate(
    config: &ProjectConfig,
    template_root: TemplateSource<'_>,
    output_dir: &Path,
) -> Result<()> {
    // Generate workspace Cargo.toml
    render_file_from_template(
        config,
        template_root.clone(),
        "root_project/Cargo.toml.tera",
        output_dir.join("Cargo.toml"),
    )
    .await?;
    // Generate .env.example
    render_file_from_template(
        config,
        template_root.clone(),
        "root_project/.env.example.tera",
        output_dir.join(".env.example"),
    )
    .await?;
    // Generate .gitignore
    render_file_from_template(
        config,
        template_root.clone(),
        "root_project/.gitignore.tera",
        output_dir.join(".gitignore"),
    )
    .await?;
    // Generate README.md
    render_file_from_template(
        config,
        template_root.clone(),
        "common/README.md.tera",
        output_dir.join("README.md"),
    )
    .await?;
    // Generate STRUCTURE.md
    render_file_from_template(
        config,
        template_root.clone(),
        "common/STRUCTURE.md.tera",
        output_dir.join("STRUCTURE.md"),
    )
    .await?;

    // Generate AI Agent folders based on selection
    for agent in &config.ai_agents {
        let folder_name = match agent {
            AIAgent::Claude => ".claude",
            AIAgent::Gemini => ".gemini",
            AIAgent::GPT => ".gpt",
        };

        let agent_dir = output_dir.join(folder_name);
        fs::create_dir_all(&agent_dir).await?;

        let template_path = format!("root_project/{}/README.md.tera", folder_name);
        let output_path = agent_dir.join("README.md");
        render_file_from_template(config, template_root.clone(), &template_path, output_path)
            .await?;
    }

    Ok(())
}
