pub mod core_crate;
pub mod application_crate;
pub mod infrastructure_crate;
pub mod api_crate;
pub mod root_crate;
pub mod frontend;
pub mod common;
pub mod utils;

use crate::config::ProjectConfig;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;
pub use utils::TemplateSource;

pub struct Generator<'a> {
    config: ProjectConfig,
    template_root: TemplateSource<'a>,
}

impl<'a> Generator<'a> {
    pub fn new(config: ProjectConfig, template_root: TemplateSource<'a>) -> Self {
        Self {
            config,
            template_root,
        }
    }

    pub async fn generate(&self, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).await.context("Failed to create output directory")?;

        // 1. Generate Root Workspace files
        root_crate::generate(&self.config, self.template_root.clone(), output_dir).await?;

        // 2. Generate Core Crate
        let core_crate_output_dir = output_dir.join("core");
        fs::create_dir_all(&core_crate_output_dir).await?;
        core_crate::generate(&self.config, self.template_root.clone(), &core_crate_output_dir).await?;

        // 3. Generate Application Crate
        let application_crate_output_dir = output_dir.join("application");
        fs::create_dir_all(&application_crate_output_dir).await?;
        application_crate::generate(&self.config, self.template_root.clone(), &application_crate_output_dir).await?;

        // 4. Generate Infrastructure Crate
        let infrastructure_crate_output_dir = output_dir.join("infrastructure");
        fs::create_dir_all(&infrastructure_crate_output_dir).await?;
        infrastructure_crate::generate(&self.config, self.template_root.clone(), &infrastructure_crate_output_dir).await?;

        // 5. Generate API Crate
        let api_crate_output_dir = output_dir.join("api");
        fs::create_dir_all(&api_crate_output_dir).await?;
        api_crate::generate(&self.config, self.template_root.clone(), &api_crate_output_dir).await?;

        Ok(())
    }
}
