use rhupster_core::config::{
    Authentication, Database, DevOps, Frontend, Infrastructure, Orm, ProjectConfig, RouterStrategy, ApiUi,
};
use rhupster_core::generator::Generator;
use std::path::PathBuf;
use tokio::fs;
use assert_cmd::Command;

#[tokio::test]
async fn test_generate_full_stack_postgres_sqlx() {
    let output_dir = PathBuf::from("test_output/multi_crate_postgres_sqlx");
    let template_root = PathBuf::from("../templates"); // Relative to rhupster-core crate root

    // Clean up
    let _ = fs::remove_dir_all(&output_dir).await;

    let config = ProjectConfig {
        name: "test-app".to_string(),
        database: Database::Postgres,
        orm: Orm::Sqlx,
        infrastructure: vec![Infrastructure::Redis],
        frontend: Frontend::React,
        authentication: Authentication::Jwt,
        devops: DevOps { docker_compose: true },
        router_strategy: RouterStrategy::Standard,
        api_ui: ApiUi::Swagger,
        hateoas: false, // Default to false for simplicity in basic test
    };

    let generator = Generator::new(config, template_root);
    
    // Execute generation
    let result = generator.generate(&output_dir).await;
    
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());
    
    // --- Verify multi-crate project structure ---
    assert!(output_dir.join("Cargo.toml").exists()); // Workspace Cargo.toml
    assert!(output_dir.join("core/Cargo.toml").exists());
    assert!(output_dir.join("application/Cargo.toml").exists());
    assert!(output_dir.join("infrastructure/Cargo.toml").exists());
    assert!(output_dir.join("api/Cargo.toml").exists());
    
    // Verify some key files exist in sub-crates
    assert!(output_dir.join("core/src/domain/user.rs").exists());
    assert!(output_dir.join("application/src/services/user_service.rs").exists());
    assert!(output_dir.join("infrastructure/src/persistence/user_adapter.rs").exists());
    assert!(output_dir.join("api/src/main.rs").exists());
    assert!(output_dir.join("api/client/package.json").exists()); // Frontend in api/client

    // --- Attempt to compile the generated project ---
    println!("Attempting to compile the generated project...");
    let mut cmd = Command::cargo_bin("cargo").unwrap(); // Use assert_cmd to run cargo
    cmd.current_dir(&output_dir) // Run cargo from the generated project root
        .arg("build")
        .assert()
        .success();

    println!("Generated project compiled successfully!");
}