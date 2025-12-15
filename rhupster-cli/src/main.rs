use anyhow::Result;
use clap::Parser;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select, Confirm};
use rhupster_core::config::{
    Authentication,
    Database,
    DevOps,
    Frontend,
    Infrastructure,
    OAuthProvider,
    ProjectConfig,
};
use rhupster_core::generator::Generator;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory for the new project
    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    /// Path to the templates directory (defaults to ./templates)
    #[arg(short, long, default_value = "templates")]
    templates: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", style("Welcome to Rhupster - The Rust/Axum Generator").bold().cyan());
    println!("Let's configure your new project.\n");

    let theme = ColorfulTheme::default();

    // 1. Project Name
    let name: String = Input::with_theme(&theme)
        .with_prompt("What is your project name?")
        .default("my-axum-app".into())
        .interact_text()?;

    // 2. Database
    let db_opts = vec![Database::Postgres, Database::MySQL, Database::MongoDB, Database::SQLite];
    let db_idx = Select::with_theme(&theme)
        .with_prompt("Select a Database")
        .default(0)
        .items(&db_opts)
        .interact()?;
    let database = db_opts[db_idx];

    // 3. Infrastructure
    let infra_opts = vec![Infrastructure::Redis, Infrastructure::Kafka];
    let infra_idxs = MultiSelect::with_theme(&theme)
        .with_prompt("Select Infrastructure & Caching (Space to select)")
        .items(&infra_opts)
        .interact()?;
    let infrastructure = infra_idxs.iter().map(|&i| infra_opts[i]).collect();

    // 4. Frontend
    let fe_opts = vec![Frontend::React, Frontend::Vue, Frontend::Svelte, Frontend::Angular, Frontend::None];
    let fe_idx = Select::with_theme(&theme)
        .with_prompt("Select a Frontend Framework")
        .default(0)
        .items(&fe_opts)
        .interact()?;
    let frontend = fe_opts[fe_idx];

    // 5. Authentication
    let auth_types = vec!["None", "Basic (Username/Password)", "OAuth2"];
    let auth_idx = Select::with_theme(&theme)
        .with_prompt("Select Authentication Strategy")
        .default(1)
        .items(&auth_types)
        .interact()?;

    let authentication = match auth_idx {
        0 => Authentication::None,
        1 => Authentication::Basic,
        2 => {
            let oauth_opts = vec![OAuthProvider::Discord, OAuthProvider::Google, OAuthProvider::Apple, OAuthProvider::GitHub];
            let oauth_idxs = MultiSelect::with_theme(&theme)
                .with_prompt("Select OAuth2 Providers")
                .items(&oauth_opts)
                .interact()?;
            Authentication::OAuth2(oauth_idxs.iter().map(|&i| oauth_opts[i]).collect())
        }
        _ => unreachable!(),
    };

    // 6. DevOps
    let docker_compose = Confirm::with_theme(&theme)
        .with_prompt("Generate Docker Compose?")
        .default(true)
        .interact()?;

    let config = ProjectConfig {
        name: name.clone(),
        database,
        infrastructure,
        frontend,
        authentication,
        devops: DevOps { docker_compose },
    };

    println!("\n{}", style("Configuration Complete!").green());
    println!("Generating project '{}'விற்கு...", name);

    // Determine output directory: if user passed ".", append project name.
    let output_path = if args.output.to_string_lossy() == "." {
        env::current_dir()?.join(&name)
    } else {
        args.output
    };

    let generator = Generator::new(config, args.templates);
    generator.generate(&output_path).await?;

    println!("\n{}", style("Success! Project generated.").bold().green());
    println!("cd {}", output_path.display());
    println!("cargo run");

    Ok(())
}