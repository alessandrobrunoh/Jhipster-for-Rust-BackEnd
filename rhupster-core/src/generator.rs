use crate::config::{ProjectConfig, Frontend, Authentication, RouterStrategy, Orm};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::{Tera, Context as TeraContext};
use tokio::fs;
use walkdir::WalkDir;
use std::time::{SystemTime, UNIX_EPOCH};

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
        self.generate_migrations(output_dir).await?;
        self.generate_common(output_dir).await?;

        Ok(())
    }

    async fn generate_common(&self, output_dir: &Path) -> Result<()> {
        let common_dir = self.template_root.join("common");
        if common_dir.exists() {
             self.render_dir_recursive(&common_dir, output_dir).await?;
        }
        Ok(())
    }

    async fn generate_migrations(&self, output_dir: &Path) -> Result<()> {
        if self.config.orm == Orm::None {
            return Ok(());
        }

        println!("Generating Migrations...");
        let migrations_dir = output_dir.join("migrations");
        fs::create_dir_all(&migrations_dir).await?;
        
        // Simple timestamp generation
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();

        let context = self.create_context();
        let mut tera = Tera::default();
        let init_sql_path = self.template_root.join("backend/migrations/init.sql.tera");
        
        if init_sql_path.exists() {
            tera.add_template_file(&init_sql_path, Some("init.sql"))?;
            let rendered = tera.render("init.sql", &context)?;

            if self.config.orm == Orm::Sqlx {
                let file_name = format!("{}_init.sql", timestamp);
                fs::write(migrations_dir.join(file_name), rendered).await?;
            } else if self.config.orm == Orm::Diesel {
                // Diesel often prefers YYYY-MM-DD pattern, but CLI handles creating new ones. 
                // Let's use a fixed folder name for init to be safe with Diesel format or just suggest users run diesel setup.
                // Actually, let's try to mimic Diesel format: YYYY-MM-DD-HHMMSS_init
                let diesel_folder_name = format!("{}_init", chrono::Local::now().format("%Y-%m-%d-%H%M%S"));
                let diesel_migration_dir = migrations_dir.join(diesel_folder_name);
                fs::create_dir_all(&diesel_migration_dir).await?;
                
                fs::write(diesel_migration_dir.join("up.sql"), rendered).await?;
                fs::write(diesel_migration_dir.join("down.sql"), "-- Undo init").await?;
            }
        }
        
        Ok(())
    }

    async fn generate_backend(&self, output_dir: &Path) -> Result<()> {
        println!("Generating Backend...");
        let backend_template_dir = self.template_root.join("backend");
        
        // 1. Render core backend files (excluding features folder)
        // We use a custom walker or just standard recursion but skip 'features'
        self.render_dir_recursive_filtering(&backend_template_dir, output_dir, "features").await?;

        // 2. Handle Routing Strategy
        match self.config.router_strategy {
            RouterStrategy::AxumFolderRouter => {
                let strategy_dir = backend_template_dir.join("features/folder");
                if strategy_dir.exists() {
                     self.render_dir_recursive(&strategy_dir, &output_dir.join("src")).await?;
                }
            },
            _ => {
                // Standard or Controller
                let strategy_dir = backend_template_dir.join("features/controller");
                if strategy_dir.exists() {
                     self.render_dir_recursive(&strategy_dir, &output_dir.join("src")).await?;
                }
            }
        }

        Ok(())
    }

    async fn render_dir_recursive_filtering(&self, src: &Path, dst: &Path, ignore_dir: &str) -> Result<()> {
        let context = self.create_context();

        for entry in WalkDir::new(src).min_depth(1) {
            let entry = entry?;
            let relative_path = entry.path().strip_prefix(src)?;
            
            // Skip ignored directory
            if relative_path.starts_with(ignore_dir) {
                continue;
            }

            let target_path = dst.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(target_path).await?;
            } else {
                let file_name = entry.file_name().to_string_lossy();
                if file_name.ends_with(".tera") {
                    let new_file_name = file_name.trim_end_matches(".tera");
                    let final_target_path = target_path.with_file_name(new_file_name);
                    
                    let mut local_tera = Tera::default();
                    local_tera.add_template_file(entry.path(), Some("temp"))?;
                    let rendered = local_tera.render("temp", &context)?;
                    fs::write(final_target_path, rendered).await?;
                } else {
                    fs::copy(entry.path(), target_path).await?;
                }
            }
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
            fs::write(client_dir.join("README.md"), format!("Placeholder for {} project", frontend_type)).await?;
            return Ok(());
        }

        self.render_dir_recursive(&template_dir, &client_dir).await?;

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
        context.insert("orm", &self.config.orm.to_string().to_lowercase());
        context.insert("hateoas", &self.config.hateoas);
        
        let infra_strings: Vec<String> = self.config.infrastructure.iter().map(|i| i.to_string().to_lowercase()).collect();
        context.insert("infrastructure", &infra_strings);
        
        context.insert("frontend", &self.config.frontend.to_string().to_lowercase());
        context.insert("router_strategy", &self.config.router_strategy.to_string());
        
        match &self.config.authentication {
            Authentication::Basic => {
                context.insert("auth_type", "basic");
            },
            Authentication::Jwt => {
                context.insert("auth_type", "jwt");
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

    async fn render_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        let context = self.create_context();

        for entry in WalkDir::new(src).min_depth(1) {
            let entry = entry?;
            let relative_path = entry.path().strip_prefix(src)?;
            let target_path = dst.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(target_path).await?;
            } else {
                let file_name = entry.file_name().to_string_lossy();
                if file_name.ends_with(".tera") {
                    let new_file_name = file_name.trim_end_matches(".tera");
                    let final_target_path = target_path.with_file_name(new_file_name);
                    
                    let mut local_tera = Tera::default();
                    local_tera.add_template_file(entry.path(), Some("temp"))?;
                    let rendered = local_tera.render("temp", &context)?;
                    fs::write(final_target_path, rendered).await?;
                } else {
                    fs::copy(entry.path(), target_path).await?;
                }
            }
        }
        Ok(())
    }
}