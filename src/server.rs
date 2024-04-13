use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tictactoe::{error::Result, Game};
use tokio::net::TcpListener;
use tracing::{info, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

#[tracing::instrument(name = "Creating a new game", skip(db_pool))]
async fn new_game(State(db_pool): State<PgPool>) -> Result<String> {
    let id = Uuid::new_v4();
    info!("Id: {}", id.to_string());
    sqlx::query("INSERT INTO games(id, state) VALUES ($1, $2)")
        .bind(id)
        .bind(Game::default())
        .execute(&db_pool)
        .await?;
    Ok(id.to_string())
}

#[tracing::instrument(name = "Fetching details of the game", skip(db_pool))]
async fn get_game(Path(id): Path<Uuid>, State(db_pool): State<PgPool>) -> Result<Json<Game>> {
    let game: Game = sqlx::query_as("SELECT state FROM games WHERE id=$1")
        .bind(id)
        .fetch_one(&db_pool)
        .await?;

    Ok(Json(game))
}

#[tracing::instrument(name = "Performing the move", skip(db_pool))]
async fn make_move(
    Path(id): Path<Uuid>,
    State(db_pool): State<PgPool>,
    Json((row, column)): Json<(usize, usize)>,
) -> Result<()> {
    let mut game: Game = sqlx::query_as("SELECT state FROM games WHERE id=$1")
        .bind(id)
        .fetch_one(&db_pool)
        .await?;

    game.make_turn(row, column).unwrap();

    sqlx::query("UPDATE games SET state=$2 WHERE id=$1")
        .bind(id)
        .bind(game)
        .execute(&db_pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("tictactoe".to_owned(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let db_pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::new()
                .host("127.0.0.1")
                .username("postgres")
                .password("password")
                .port(5432)
                .database("tictactoe"),
        )
        .await
        .unwrap();
    let app = Router::new()
        .route("/game", get(new_game))
        .route("/game/:id", get(get_game).post(make_move))
        .with_state(db_pool);

    let listener = TcpListener::bind("127.0.0.1:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
