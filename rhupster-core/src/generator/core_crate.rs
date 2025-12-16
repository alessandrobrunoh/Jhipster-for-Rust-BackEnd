use crate::config::ProjectConfig;
use anyhow::{Result, anyhow};
use std::path::Path;
use super::utils::{render_dir_recursive, TemplateSource};

pub async fn generate(config: &ProjectConfig, template_root: TemplateSource<'_>, output_dir: &Path) -> Result<()> {
    // Copy core crate templates
    let src_template_dir = template_root.join("core").ok_or_else(|| anyhow!("Template directory 'core' not found"))?;
    render_dir_recursive(config, src_template_dir, output_dir).await?;
    Ok(())
}
