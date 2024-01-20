use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use axum::routing::post;
use serde::{Serialize, Deserialize};
use tower_http::{services::ServeDir, compression::CompressionLayer};
use axum::{response::IntoResponse, routing::get, Router};
use tower_livereload::LiveReloadLayer;
use surrealdb::Surreal;
use surrealdb::engine::local::{Mem, Db};

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    posts: Vec<Post>,
}

async fn index(State(db): State<Surreal<Db>>) -> impl IntoResponse {
    // Get all posts
    let posts: Vec<Post> = db.select("post").await.unwrap();
    
    Index {
        posts
    }
}

async fn insert(State(db): State<Surreal<Db>>, Form(post): Form<Post>) -> impl IntoResponse {
    // Get all posts
    let _: Vec<Post> = db.create("post").content(post).await.unwrap();
    Redirect::to("/")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::fmt::init();

    // Database
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("blazy").use_db("main").await?;

    // Setup router
    let app = Router::new()
        .route("/", get(index))
        .route("/insert", post(insert))
        .layer(LiveReloadLayer::new())
        .layer(CompressionLayer::new())
        .nest_service("/static", ServeDir::new("static/"))
        .with_state(db);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Started server at http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}
