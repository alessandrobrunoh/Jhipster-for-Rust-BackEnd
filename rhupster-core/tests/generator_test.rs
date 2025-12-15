use rhupster_core::config::{
    Authentication, Database, DevOps, Frontend, Infrastructure, Orm, ProjectConfig, RouterStrategy,
};
use rhupster_core::generator::Generator;
use std::path::PathBuf;
use tokio::fs;

#[tokio::test]
async fn test_generate_full_stack_postgres_sqlx() {
    let output_dir = PathBuf::from("test_output/postgres_sqlx");
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
        hateoas: true,
    };

    let generator = Generator::new(config, template_root);
    
    // Execute
    let result = generator.generate(&output_dir).await;
    
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());
    
    // Assert files exist
    assert!(output_dir.join("Cargo.toml").exists());
    assert!(output_dir.join("src/main.rs").exists());
    assert!(output_dir.join("src/domain/truck.rs").exists());
    assert!(output_dir.join("client/package.json").exists());
    assert!(output_dir.join("migrations").exists());
}
