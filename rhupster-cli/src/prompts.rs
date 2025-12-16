use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use rhupster_core::config::{
    AIAgent, ApiUi, Authentication, Database, DevOps, Frontend, Infrastructure, OAuthProvider, Orm,
    ProjectConfig, RouterStrategy,
};

pub struct PromptService {
    theme: ColorfulTheme,
}

impl PromptService {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    pub fn collect_config(&self) -> Result<ProjectConfig> {
        let name = self.ask_name()?;
        let database = self.ask_database()?;
        let orm = self.ask_orm(database)?;
        let infrastructure = self.ask_infrastructure()?;
        let router_strategy = self.ask_router_strategy()?;
        let api_ui = self.ask_api_ui()?;
        let frontend = self.ask_frontend()?;
        let authentication = self.ask_authentication()?;
        let hateoas = self.ask_hateoas()?;
        let docker_compose = self.ask_docker_compose()?;
        let ai_agents = self.ask_ai_agents()?;

        Ok(ProjectConfig {
            name,
            database,
            orm,
            infrastructure,
            frontend,
            authentication,
            devops: DevOps { docker_compose },
            router_strategy,
            api_ui,
            hateoas,
            ai_agents,
        })
    }

    fn ask_name(&self) -> Result<String> {
        Input::with_theme(&self.theme)
            .with_prompt("What is your project name?")
            .default("my-axum-app".into())
            .interact_text()
            .map_err(Into::into)
    }

    fn ask_database(&self) -> Result<Database> {
        let db_opts = vec![
            Database::Postgres,
            Database::MySQL,
            Database::MongoDB,
            Database::SQLite,
        ];
        let idx = Select::with_theme(&self.theme)
            .with_prompt("Select a Database")
            .default(0)
            .items(&db_opts)
            .interact()?;
        Ok(db_opts[idx])
    }

    fn ask_orm(&self, database: Database) -> Result<Orm> {
        if database == Database::MongoDB {
            println!("  (MongoDB selected: skipping ORM selection, using native driver)");
            Ok(Orm::None)
        } else {
            let orm_opts = vec![Orm::Sqlx, Orm::Diesel, Orm::SeaOrm]; // Added SeaOrm
            let idx = Select::with_theme(&self.theme)
                .with_prompt("Select an ORM")
                .default(0)
                .items(&orm_opts)
                .interact()?;
            Ok(orm_opts[idx])
        }
    }

    fn ask_infrastructure(&self) -> Result<Vec<Infrastructure>> {
        let infra_opts = vec![
            Infrastructure::Redis,
            Infrastructure::Kafka,
            Infrastructure::Socket,
        ];
        let idxs = MultiSelect::with_theme(&self.theme)
            .with_prompt("Select Infrastructure & Caching")
            .items(&infra_opts)
            .interact()?;
        Ok(idxs.iter().map(|&i| infra_opts[i]).collect())
    }

    fn ask_router_strategy(&self) -> Result<RouterStrategy> {
        let opts = vec![
            RouterStrategy::Standard,
            RouterStrategy::AxumController,
            RouterStrategy::AxumFolderRouter,
        ]; // Reintroduced AxumFolderRouter
        let idx = Select::with_theme(&self.theme)
            .with_prompt("Select Routing Strategy")
            .default(0)
            .items(&opts)
            .interact()?;
        Ok(opts[idx])
    }

    fn ask_api_ui(&self) -> Result<ApiUi> {
        let opts = vec![ApiUi::Swagger, ApiUi::Scalar, ApiUi::None];
        let idx = Select::with_theme(&self.theme)
            .with_prompt("Select API Documentation UI")
            .default(0)
            .items(&opts)
            .interact()?;
        Ok(opts[idx])
    }

    fn ask_frontend(&self) -> Result<Frontend> {
        let opts = vec![
            Frontend::React,
            Frontend::Vue,
            Frontend::Svelte,
            Frontend::Angular,
            Frontend::None,
        ];
        let idx = Select::with_theme(&self.theme)
            .with_prompt("Select a Frontend Framework")
            .default(0)
            .items(&opts)
            .interact()?;
        Ok(opts[idx])
    }

    fn ask_authentication(&self) -> Result<Authentication> {
        let types = vec!["None", "Basic", "JWT (User/Role System)", "OAuth2"];
        let idx = Select::with_theme(&self.theme)
            .with_prompt("Select Authentication Strategy")
            .default(2)
            .items(&types)
            .interact()?;

        match idx {
            0 => Ok(Authentication::None),
            1 => Ok(Authentication::Basic),
            2 => Ok(Authentication::Jwt),
            3 => {
                let opts = vec![
                    OAuthProvider::Discord,
                    OAuthProvider::Google,
                    OAuthProvider::Apple,
                    OAuthProvider::GitHub,
                ];
                let idxs = MultiSelect::with_theme(&self.theme)
                    .with_prompt("Select OAuth2 Providers")
                    .items(&opts)
                    .interact()?;
                Ok(Authentication::OAuth2(
                    idxs.iter().map(|&i| opts[i]).collect(),
                ))
            }
            _ => unreachable!(),
        }
    }

    fn ask_hateoas(&self) -> Result<bool> {
        Confirm::with_theme(&self.theme)
            .with_prompt("Enable HATEOAS (Hypermedia) support?")
            .default(false)
            .interact()
            .map_err(Into::into)
    }

    fn ask_docker_compose(&self) -> Result<bool> {
        Confirm::with_theme(&self.theme)
            .with_prompt("Generate Docker Compose?")
            .default(true)
            .interact()
            .map_err(Into::into)
    }

    fn ask_ai_agents(&self) -> Result<Vec<AIAgent>> {
        let opts = vec![AIAgent::Claude, AIAgent::Gemini, AIAgent::GPT];
        let idxs = MultiSelect::with_theme(&self.theme)
            .with_prompt("Select AI Agents (for future context folders)")
            .items(&opts)
            .interact()?;
        Ok(idxs.iter().map(|&i| opts[i]).collect())
    }
}
