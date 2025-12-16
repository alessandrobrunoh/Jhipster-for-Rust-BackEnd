use anyhow::Result;
use clap::Parser;
use console::style;
use rhupster_core::generator::{Generator, TemplateSource};
use std::env;
use std::path::PathBuf;
use include_dir::{include_dir, Dir};

mod prompts;
use prompts::PromptService;

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/../templates");

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

    println!("{}", style("Welcome to Rhupster - The Rust/Axum Enterprise Generator").bold().cyan());
    println!("Let's configure your new project.\n");

    let prompts = PromptService::new();
    let config = prompts.collect_config()?;

    println!("\n{}", style("Configuration Complete!").green());
    println!("Generating project '{}'...", config.name);
    println!("  - Database: {}", config.database);
    println!("  - ORM: {}", config.orm);
    println!("  - Auth: {:?}", config.authentication);
    println!("  - Domain: Trucks & Products included.");

    // Determine output directory
    let output_path = if args.output.to_string_lossy() == "." {
        env::current_dir()?.join(&config.name)
    } else {
        args.output
    };

    let template_source = if args.templates.exists() {
        println!("Using local templates from: {}", args.templates.display());
        TemplateSource::Path(args.templates)
    } else {
        println!("Using embedded templates.");
        TemplateSource::Embedded(&TEMPLATES)
    };

    let generator = Generator::new(config, template_source);
    generator.generate(&output_path).await?;

    println!("\n{}", style("Success! Project generated.").bold().green());
    println!("cd {}", output_path.display());
    println!("cargo run");

    Ok(())
}