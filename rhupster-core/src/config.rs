use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub database: Database,
    pub infrastructure: Vec<Infrastructure>,
    pub frontend: Frontend,
    pub authentication: Authentication,
    pub devops: DevOps,
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
pub enum Infrastructure {
    Redis,
    Kafka,
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
    None, // Fallback
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
