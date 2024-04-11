use tictactoe::{board::Board, error::Result, game::game::Game, mark::Mark};

use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tokio::net::TcpListener;
use uuid::Uuid;

async fn new_game(State(db_pool): State<PgPool>) -> Result<String> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO games(id, state) VALUES ($1, $2)")
        .bind(id)
        .bind(Game::default())
        .execute(&db_pool)
        .await?;
    Ok(id.to_string())
}

async fn get_game(
    Path(id): Path<Uuid>,
    State(db_pool): State<PgPool>,
) -> Result<impl IntoResponse> {
    let game: Game = sqlx::query_as("SELECT state FROM games WHERE id=$1")
        .bind(id)
        .fetch_one(&db_pool)
        .await?;

    Ok(Json(game))
}

#[debug_handler]
async fn make_move(
    Path(id): Path<Uuid>,
    State(db_pool): State<PgPool>,
    Json((row, column)): Json<(usize, usize)>,
) -> Result<impl IntoResponse> {
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

    Ok("ok")
}

#[tokio::main]
async fn main() {
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
