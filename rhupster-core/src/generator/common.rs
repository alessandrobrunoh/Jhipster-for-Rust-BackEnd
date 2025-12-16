use crate::config::ProjectConfig;
use anyhow::Result;
use std::path::Path;
use super::utils::{render_dir_recursive, TemplateSource};

pub async fn generate(config: &ProjectConfig, template_root: TemplateSource<'_>, output_dir: &Path) -> Result<()> {
    let common_dir = template_root.join("common");
    
    let exists = common_dir.as_ref().map(|t| t.exists()).unwrap_or(false);

    if exists {
         render_dir_recursive(config, common_dir.unwrap(), output_dir).await?;
    }
    Ok(())
}
