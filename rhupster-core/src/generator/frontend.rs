use crate::config::{ProjectConfig, Frontend};
use anyhow::Result;
use std::path::Path;
use tokio::fs;
use super::utils::{render_dir_recursive, TemplateSource};

pub async fn generate(config: &ProjectConfig, template_root: TemplateSource<'_>, output_dir: &Path) -> Result<()> {
    println!("Generating Frontend...");
    // The output_dir passed here is already the destination (e.g., .../api/client). 
    // We should NOT append another "client".
    let client_dir = output_dir;
    fs::create_dir_all(&client_dir).await?;

    let frontend_type = match config.frontend {
        Frontend::React => "react",
        Frontend::Vue => "vue",
        Frontend::Svelte => "svelte",
        Frontend::Angular => "angular",
        Frontend::None => return Ok(()),
    };

    let template_dir = template_root.join("frontend").and_then(|t| t.join(frontend_type));
    
    let exists = template_dir.as_ref().map(|t| t.exists()).unwrap_or(false);

    if !exists {
        println!("Warning: No template found for {}", frontend_type);
        fs::write(client_dir.join("README.md"), format!("Placeholder for {} project", frontend_type)).await?;
        return Ok(());
    }

    // safe unwrap because exists is true
    render_dir_recursive(config, template_dir.unwrap(), &client_dir).await?;

    Ok(())
}
