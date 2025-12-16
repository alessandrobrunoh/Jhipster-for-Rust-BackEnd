# RustHipster

Generatore opinionated per progetti web in Rust ispirato a **JHipster**: un’unica CLI, un wizard interattivo e una DSL di dominio per descrivere entità e relazioni. Produce scaffolding completo per backend in **Axum** o **Actix**, database e migrazioni, sicurezza, osservabilità e integrazioni opzionali.

## Obiettivo
Offrire uno stack produttivo “ready-to-build” che standardizza convenzioni e riduce il boilerplate iniziale, mantenendo estendibilità tramite plugin/blueprints e rigenerazione controllata del codice generato.

## Caratteristiche chiave
- Framework runtime selezionabile: Axum o Actix, con template dedicati ma DSL comune.
- DSL di dominio (JDL-like) per definire entità, campi, relazioni, validazioni, indici.
- Generazione automatica di modelli, repository SQLx, migrazioni, handler REST e routing.
- Sicurezza: **JWT** con refresh, ruoli/permessi; estensioni OAuth2 (opzionale).
- Observability: health, metrics Prometheus, logging strutturato con tracing.
- Configurazione ambienti: profili dev/prod, .env, Dockerfile multi-stage, Docker Compose.
- Frontend opzionale (SvelteKit) integrabile con proxy/gateway e i18n.
- OpenAPI (utoipa) sincronizzata con gli handler generati (opzionale).
- Sistemi di caching/ratelimiting (Redis) e job di background (plugin).
- Struttura “user vs generated code” per permettere rigenerazioni senza perdere modifiche.

## Architettura del progetto generato
- Workspace Cargo con crate:
  - backend: runtime Axum/Actix, routing, handlers, middleware, stato applicativo.
  - migration: runner delle migrazioni SQL (SQLx) e orchestrazione delle versioni.
  - common: costanti, errori, utilità condivise tra moduli.
- Directory convenzionate:
  - src/models: struct generate dal DSL.
  - src/repo: repository SQLx per CRUD/queries.
  - src/handlers: HTTP handlers e validazioni.
  - src/router: router principale + snippet generati.
  - migration/sql: file di migrazione generati e ordinati.
- Stato applicativo e configurazioni:
  - AppState con pool DB, cache e servizi.
  - Config caricata da env e profili, override per CI/CD.

## DSL di dominio (schema previsto)
- Sintassi dichiarativa per:
  - Entity, campi (tipi: string, int, long, uuid, bool, datetime, enum), default, required.
  - Relazioni: one-to-one, one-to-many, many-to-many.
  - Indici/unique, foreign keys, cascade rules.
  - Validazioni: lunghezze, pattern, range, custom validators.
- Output della DSL:
  - Modelli Rust, migrazioni SQL, repository, handlers REST, snippet router, documentazione OpenAPI.

## Sicurezza e middleware
- JWT: access + refresh, rotazione, blacklist opzionale.
- Ruoli e autorizzazioni applicate per rotta/handler.
- CORS, rate limiting, compressione, error handling uniforme.
- Audit logging per operazioni sensibili.

## DevOps e ambienti
- Dev: hot reload, tracing verboso, DB locale (SQLite default).
- Prod: build minimale, configurazione tramite env, **Docker** e Compose (Postgres + Redis).
- CI/CD: test unit/integration, lint, build, migrazioni, generazione OpenAPI e publish artefatti.

## Estendibilità
- Blueprints/Plugin:
  - OAuth2/Keycloak, Admin UI, Mailer, Storage S3, Task scheduler, WebSocket.
- Hook di generazione:
  - Post-process per formatter, linter, injection di snippet custom.
- Versionamento template:
  - Migrazioni di progetto e “codemods” per upgrade tra versioni.

## Filosofia
- “Convenzione prima della configurazione”: scelte opinionated per velocità.
- “Generated separato da manuale”: rigenerazioni sicure e prevedibili.
- “Portabilità”: runtime intercambiabili (Axum/Actix) senza cambiare la DSL.
- Next: JWT completo, OpenAPI, validazioni avanzate, relazioni, plugin Redis/Jobs.
- Futuro: gateway/API composition, admin auto-generato, analytics

# Example
```bash
Welcome to Rhupster - The Rust/Axum Enterprise Generator
Let's configure your new project.

✔ What is your project name? · my-axum-app
✔ Select a Database · Postgres
✔ Select an ORM · Sqlx
✔ Select Infrastructure & Caching ·
✔ Select Routing Strategy · AxumController
✔ Select API Documentation UI · Swagger
✔ Select a Frontend Framework · React
✔ Select Authentication Strategy · JWT (User/Role System)
✔ Enable HATEOAS (Hypermedia) support? · no
✔ Generate Docker Compose? · yes
✔ Select AI Agents (for future context folders) · Claude

Configuration Complete!
Generating project 'my-axum-app'...
  - Database: Postgres
  - ORM: Sqlx
  - Auth: Jwt
  - Domain: Trucks & Products included.
Using embedded templates.
Generating Frontend...

Success! Project generated.
cd /Users/tacosalfornoh/Coding/Rust/Rhupster/rhupster-cli/src/my-axum-app
cargo run
```
