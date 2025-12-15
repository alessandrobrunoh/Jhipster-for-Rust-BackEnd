use crate::config::{ProjectConfig, Frontend, Authentication};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::{Tera, Context as TeraContext};
use tokio::fs;
use walkdir::WalkDir;

pub struct Generator {
    config: ProjectConfig,
    template_root: PathBuf,
}

impl Generator {
    pub fn new(config: ProjectConfig, template_root: PathBuf) -> Self {
        Self {
            config,
            template_root,
        }
    }

    pub async fn generate(&self, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir).await.context("Failed to create output directory")?;

        self.generate_backend(output_dir).await?;
        self.generate_frontend(output_dir).await?;
        self.generate_infrastructure(output_dir).await?;

        Ok(())
    }

    async fn generate_backend(&self, output_dir: &Path) -> Result<()> {
        println!("Generating Backend...");
        let backend_template_dir = self.template_root.join("backend");
        // For simplicity in this prototype, we're assuming a flat structure or specific files.
        // In a real generic generator, we'd walk the directory.
        
        let mut tera = Tera::default();
        
        // Register templates
        let cargo_toml_path = backend_template_dir.join("Cargo.toml.tera");
        let main_rs_path = backend_template_dir.join("src/main.rs.tera");
        
        // We only add templates if they exist to avoid errors during dev if I haven't created them yet
        if cargo_toml_path.exists() {
            tera.add_template_file(&cargo_toml_path, Some("Cargo.toml"))?;
        }
        if main_rs_path.exists() {
            tera.add_template_file(&main_rs_path, Some("main.rs"))?;
        }

        let context = self.create_context();

        // Render Cargo.toml
        if cargo_toml_path.exists() {
            let rendered = tera.render("Cargo.toml", &context)?;
            fs::write(output_dir.join("Cargo.toml"), rendered).await?;
        }

        // Render main.rs
        if main_rs_path.exists() {
            let src_dir = output_dir.join("src");
            fs::create_dir_all(&src_dir).await?;
            let rendered = tera.render("main.rs", &context)?;
            fs::write(src_dir.join("main.rs"), rendered).await?;
        }

        Ok(())
    }

    async fn generate_frontend(&self, output_dir: &Path) -> Result<()> {
        println!("Generating Frontend...");
        let client_dir = output_dir.join("client");
        fs::create_dir_all(&client_dir).await?;

        let frontend_type = match self.config.frontend {
            Frontend::React => "react",
            Frontend::Vue => "vue",
            Frontend::Svelte => "svelte",
            Frontend::Angular => "angular",
            Frontend::None => return Ok(()),
        };

        let template_dir = self.template_root.join("frontend").join(frontend_type);
        
        if !template_dir.exists() {
            println!("Warning: No template found for {}", frontend_type);
            // Create a placeholder
            fs::write(client_dir.join("README.md"), format!("Placeholder for {} project", frontend_type)).await?;
            return Ok(());
        }

        // Simple copy for now (recursive)
        // In real world, we might render package.json
        self.copy_dir_recursive(&template_dir, &client_dir).await?;

        Ok(())
    }

    async fn generate_infrastructure(&self, output_dir: &Path) -> Result<()> {
         if !self.config.devops.docker_compose {
            return Ok(());
        }
        println!("Generating Infrastructure...");

        let infra_dir = self.template_root.join("infrastructure");
        let docker_compose_path = infra_dir.join("docker-compose.yml.tera");
        
        if docker_compose_path.exists() {
            let mut tera = Tera::default();
            tera.add_template_file(&docker_compose_path, Some("docker-compose.yml"))?;
            
            let context = self.create_context();
            let rendered = tera.render("docker-compose.yml", &context)?;
            
            fs::write(output_dir.join("docker-compose.yml"), rendered).await?;
        }

        Ok(())
    }

    fn create_context(&self) -> TeraContext {
        let mut context = TeraContext::new();
        context.insert("name", &self.config.name);
        context.insert("database", &self.config.database.to_string().to_lowercase());
        
        let infra_strings: Vec<String> = self.config.infrastructure.iter().map(|i| i.to_string().to_lowercase()).collect();
        context.insert("infrastructure", &infra_strings);
        
        context.insert("frontend", &self.config.frontend.to_string().to_lowercase());
        
        // Auth context
        match &self.config.authentication {
            Authentication::Basic => {
                context.insert("auth_type", "basic");
            },
            Authentication::OAuth2(providers) => {
                context.insert("auth_type", "oauth2");
                let provider_strings: Vec<String> = providers.iter().map(|p| p.to_string().to_lowercase()).collect();
                context.insert("oauth_providers", &provider_strings);
            },
            Authentication::None => {
                context.insert("auth_type", "none");
            }
        }

        context
    }

    // Helper to copy dirs
    async fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        for entry in WalkDir::new(src).min_depth(1) {
            let entry = entry?;
            let relative_path = entry.path().strip_prefix(src)?;
            let target_path = dst.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(target_path).await?;
            } else {
                fs::copy(entry.path(), target_path).await?;
            }
        }
        Ok(())
    }
}
