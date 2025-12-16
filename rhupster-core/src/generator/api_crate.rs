use crate::config::{ProjectConfig, RouterStrategy};
use anyhow::{Result, anyhow};
use std::path::Path;
use super::utils::{render_dir_recursive, render_dir_recursive_filtering, render_file_from_template, TemplateSource};
use super::frontend;

pub async fn generate(config: &ProjectConfig, template_root: TemplateSource<'_>, output_dir: &Path) -> Result<()> {
    // Stage 1: Generate base API crate files and directories
    // This copies contents from `templates/api` (excluding `router_strategies`) to `output_dir`
    let base_api_template_dir = template_root.join("api").ok_or_else(|| anyhow!("Template directory 'api' not found"))?;

    // Copy everything from `templates/api` except `router_strategies` directory
    render_dir_recursive_filtering(config, base_api_template_dir.clone(), output_dir, "router_strategies").await?;

    // Stage 2: Handle router strategy specific files and configurations
    let router_strategy_template_base_path = match config.router_strategy {
        RouterStrategy::Standard => "api/router_strategies/standard",
        RouterStrategy::AxumController => "api/router_strategies/axum_controller",
        RouterStrategy::AxumFolderRouter => "api/router_strategies/axum_folder_router",
    };
    
    let strategy_template_dir = template_root.join(router_strategy_template_base_path).ok_or_else(|| anyhow!("Template directory '{}' not found", router_strategy_template_base_path))?;

    // Render the strategy specific Cargo.toml.tera into the api crate root (my-axum-app/api/Cargo.toml)
    let strategy_cargo_toml_template_path = format!("{}/Cargo.toml.tera", router_strategy_template_base_path);
    render_file_from_template(config, template_root.clone(), &strategy_cargo_toml_template_path, output_dir.join("Cargo.toml")).await?;

    // Copy strategy specific 'src' content (e.g., controllers or routes directories) into my-axum-app/api/src
    let strategy_src_template_dir = strategy_template_dir.join("src").ok_or_else(|| anyhow!("Strategy src directory not found for '{}'", router_strategy_template_base_path))?;
    render_dir_recursive(config, strategy_src_template_dir, &output_dir.join("src")).await?;

    // Frontend generation for the api crate
    let api_frontend_dir = output_dir.join("client");
    frontend::generate(config, template_root, &api_frontend_dir).await?;
    
    Ok(())
}