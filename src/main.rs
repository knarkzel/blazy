use askama::Template;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::{response::IntoResponse, routing::{get, post}, Router, Form};

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    name: String,
}

async fn index() -> impl IntoResponse {
    Index {
        name: String::from("Ralph"),
    }
}

#[derive(Deserialize, Template)]
#[template(path = "skill.html")]
struct Skill {
    experience: String,
}

async fn skill(Form(skill): Form<Skill>) -> impl IntoResponse {
    skill
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::fmt::init();

    // Setup router
    let app = Router::new()
        .route("/", get(index))
        .route("/skill", post(skill))
        .nest_service("/static", ServeDir::new("static/"));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Started server at http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}
