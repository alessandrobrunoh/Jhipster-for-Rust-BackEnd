use crate::config::ProjectConfig;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tera::{Tera, Context as TeraContext};
use tokio::fs;
use walkdir::WalkDir;
use include_dir::Dir;

#[derive(Clone)]
pub enum TemplateSource<'a> {
    Path(PathBuf),
    Embedded(&'a Dir<'a>),
}

impl<'a> TemplateSource<'a> {
    pub fn join(&self, path: &str) -> Option<TemplateSource<'a>> {
        match self {
            TemplateSource::Path(p) => Some(TemplateSource::Path(p.join(path))),
            TemplateSource::Embedded(d) => {
                let full_path = d.path().join(path);
                d.get_dir(full_path).map(TemplateSource::Embedded)
            },
        }
    }

    pub fn exists(&self) -> bool {
        match self {
            TemplateSource::Path(p) => p.exists(),
            TemplateSource::Embedded(_) => true,
        }
    }

    pub fn has_file(&self, path: &str) -> bool {
        match self {
            TemplateSource::Path(p) => p.join(path).exists(),
            TemplateSource::Embedded(d) => {
                let full_path = d.path().join(path);
                d.get_file(full_path).is_some()
            },
        }
    }
}

pub fn create_context(config: &ProjectConfig) -> TeraContext {
    let mut context = TeraContext::new();
    context.insert("name", &config.name);
    context.insert("database", &config.database.to_string().to_lowercase());
    context.insert("orm", &config.orm.to_string().to_lowercase());
    context.insert("hateoas", &config.hateoas);
    
    let infra_strings: Vec<String> = config.infrastructure.iter().map(|i| i.to_string().to_lowercase()).collect();
    context.insert("infrastructure", &infra_strings);
    
    context.insert("frontend", &config.frontend.to_string().to_lowercase());
    context.insert("router_strategy", &config.router_strategy.to_string());
    context.insert("api_ui", &config.api_ui.to_string().to_lowercase());
    
    match &config.authentication {
        crate::config::Authentication::Basic => {
            context.insert("auth_type", "basic");
            context.insert("authentication", "basic");
        },
        crate::config::Authentication::Jwt => {
            context.insert("auth_type", "jwt");
            context.insert("authentication", "jwt");
        },
        crate::config::Authentication::OAuth2(providers) => {
            context.insert("auth_type", "oauth2");
            context.insert("authentication", "oauth2");
            let provider_strings: Vec<String> = providers.iter().map(|p| p.to_string().to_lowercase()).collect();
            context.insert("oauth_providers", &provider_strings);
        },
        crate::config::Authentication::None => {
            context.insert("auth_type", "none");
            context.insert("authentication", "none");
        }
    }

    let mut devops_map = std::collections::HashMap::new();
    devops_map.insert("docker_compose", config.devops.docker_compose);
    context.insert("devops", &devops_map);

    context
}

pub async fn render_dir_recursive(config: &ProjectConfig, src: TemplateSource<'_>, dst: &Path) -> Result<()> {
    match src {
        TemplateSource::Path(path) => render_dir_recursive_path(config, &path, dst).await,
        TemplateSource::Embedded(dir) => render_dir_recursive_embedded(config, dir, dst).await,
    }
}

pub async fn render_dir_recursive_filtering(config: &ProjectConfig, src: TemplateSource<'_>, dst: &Path, ignore_dir: &str) -> Result<()> {
     match src {
        TemplateSource::Path(path) => render_dir_recursive_path_filtering(config, &path, dst, ignore_dir).await,
        TemplateSource::Embedded(dir) => render_dir_recursive_embedded_filtering(config, dir, dst, ignore_dir).await,
    }
}


async fn render_dir_recursive_path(config: &ProjectConfig, src: &Path, dst: &Path) -> Result<()> {
    let context = create_context(config);

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

async fn render_dir_recursive_path_filtering(config: &ProjectConfig, src: &Path, dst: &Path, ignore_dir: &str) -> Result<()> {
    let context = create_context(config);

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

// Iterative helper for embedded directories
async fn process_dir_embedded(
    start_dir: &Dir<'_>,
    base_src_path: &Path,
    dst: &Path,
    context: &TeraContext,
    ignore_dir: Option<&str>
) -> Result<()> {
    let mut stack = vec![start_dir];

    while let Some(dir) = stack.pop() {
        // Iterate over files in current directory
        for file in dir.files() {
            let relative_path = file.path().strip_prefix(base_src_path).unwrap_or(file.path());
            
            if let Some(ignore) = ignore_dir {
                if relative_path.starts_with(ignore) {
                    continue;
                }
            }

            let target_path = dst.join(relative_path);
            
            // Ensure parent dir exists
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            
            let file_name = file.path().file_name().unwrap().to_string_lossy();
            if file_name.ends_with(".tera") {
                let new_file_name = file_name.trim_end_matches(".tera");
                let final_target_path = target_path.with_file_name(new_file_name);
                
                let mut local_tera = Tera::default();
                let content = String::from_utf8_lossy(file.contents());
                local_tera.add_raw_template("temp", &content)?;
                let rendered = local_tera.render("temp", context)?;
                fs::write(final_target_path, rendered).await?;
            } else {
                fs::write(target_path, file.contents()).await?;
            }
        }

        // Add subdirectories to stack
        for subdir in dir.dirs() {
             let relative_path = subdir.path().strip_prefix(base_src_path).unwrap_or(subdir.path());
             if let Some(ignore) = ignore_dir {
                if relative_path.starts_with(ignore) {
                    continue;
                }
            }
            stack.push(subdir);
        }
    }
    Ok(())
}

async fn render_dir_recursive_embedded(config: &ProjectConfig, src: &Dir<'_>, dst: &Path) -> Result<()> {
    let context = create_context(config);
    process_dir_embedded(src, src.path(), dst, &context, None).await
}

async fn render_dir_recursive_embedded_filtering(config: &ProjectConfig, src: &Dir<'_>, dst: &Path, ignore_dir: &str) -> Result<()> {
    let context = create_context(config);
    process_dir_embedded(src, src.path(), dst, &context, Some(ignore_dir)).await
}


pub async fn render_file_from_template(config: &ProjectConfig, template_root: TemplateSource<'_>, template_path_str: &str, output_path: PathBuf) -> Result<()> {
    let context = create_context(config);
    let mut tera = Tera::default();
    
    match template_root {
        TemplateSource::Path(root) => {
             let template_file_path = root.join(template_path_str);
             tera.add_template_file(&template_file_path, Some(template_path_str))?;
             let rendered = tera.render(template_path_str, &context)?;
             fs::write(output_path, rendered).await?;
        },
        TemplateSource::Embedded(root) => {
             // template_path_str is like "root_project/Cargo.toml.tera"
             // root is the base embedded dir.
             // We assume template_path_str is relative to root.
             
             let file = root.get_file(template_path_str).context(format!("Template not found: {}", template_path_str))?;
             let content = String::from_utf8_lossy(file.contents());
             
             tera.add_raw_template(template_path_str, &content)?;
             let rendered = tera.render(template_path_str, &context)?;
             fs::write(output_path, rendered).await?;
        }
    }
    Ok(())
}