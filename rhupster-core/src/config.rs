use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub database: Database,
    pub orm: Orm,
    pub infrastructure: Vec<Infrastructure>,
    pub frontend: Frontend,
    pub authentication: Authentication,
    pub devops: DevOps,
    pub router_strategy: RouterStrategy,
    pub api_ui: ApiUi,
    pub hateoas: bool,
    pub ai_agents: Vec<AIAgent>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Database {
    Postgres,
    MySQL,
    MongoDB,
    SQLite,
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Orm {
    Sqlx,
    Diesel,
    SeaOrm, // Added SeaOrm
    None,
}

impl std::fmt::Display for Orm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Infrastructure {
    Redis,
    Kafka,
    Socket,
}

impl std::fmt::Display for Infrastructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Frontend {
    React,
    Vue,
    Svelte,
    Angular,
    None,
}

impl std::fmt::Display for Frontend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Authentication {
    None,
    Basic,
    Jwt,
    OAuth2(Vec<OAuthProvider>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OAuthProvider {
    Discord,
    Google,
    Apple,
    GitHub,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOps {
    pub docker_compose: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RouterStrategy {
    Standard,
    AxumController,
    AxumFolderRouter, // Reintroduced AxumFolderRouter
}

impl std::fmt::Display for RouterStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ApiUi {
    Swagger,
    Scalar,
    None,
}

impl std::fmt::Display for ApiUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AIAgent {
    Claude,
    Gemini,
    GPT,
}

impl std::fmt::Display for AIAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
