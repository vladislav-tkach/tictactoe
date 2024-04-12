use std::{io::stdin, str::FromStr, thread, time::Duration};

use tictactoe::{Game, Mark, State};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("Create game (1) or join existing one (2)?");
    let mut buffer = String::default();
    stdin()
        .read_line(&mut buffer)
        .expect("failed to read the choice");
    let choice: u32 = buffer
        .trim()
        .parse()
        .expect(format!("expected a number, got {buffer}").as_str());

    let server_url = "http://127.0.0.1:80/game";
    let client = reqwest::Client::new();

    let game_id: Uuid = match choice {
        1 => {
            let string = client
                .get(server_url)
                .send()
                .await
                .expect("failed to request a new game")
                .text()
                .await
                .expect("failed to get request body");
            let id = Uuid::from_str(&string).expect("failed to parse UUID");
            println!("game id: {id}");
            id
        }
        2 => {
            println!("Game id:");
            let mut buffer = String::default();
            stdin()
                .read_line(&mut buffer)
                .expect("failed to read the id");
            let id: Uuid = buffer
                .trim()
                .parse()
                .expect(format!("expected a UUID, got {buffer}").as_str());
            id
        }
        other => panic!("expected 1 or 2, got {other}"),
    };

    println!("Circle (1) or cross (2)?");
    let mut buffer = String::default();
    stdin()
        .read_line(&mut buffer)
        .expect("failed to read the role");
    let choice: u32 = buffer
        .trim()
        .parse()
        .expect(format!("expected a number, got {buffer}").as_str());
    let player_role: Mark = match choice {
        1 => Mark::Circle,
        2 => Mark::Cross,
        other => panic!("expected 1 or 2, got {other}"),
    };

    loop {
        let game_url = format!("{server_url}/{}", game_id.to_string());
        let game: Game = client
            .get(game_url.as_str())
            .send()
            .await
            .expect("failed to request game")
            .json()
            .await
            .expect("failed to get request body");

        match game.state() {
            State::Playing(active_role) => {
                if active_role != player_role {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

                println!("{}", game.board());

                println!("Enter your move (row, column)");
                let mut buffer = String::default();
                stdin()
                    .read_line(&mut buffer)
                    .expect("failed to read the move");
                let mut parts = buffer.split(' ');
                let row: usize = parts
                    .next()
                    .expect(format!("expected two numbers, got {buffer}").as_str())
                    .trim()
                    .parse()
                    .expect(format!("expected two numbers, got {buffer}").as_str());
                let column: usize = parts
                    .next()
                    .expect(format!("expected two numbers, got {buffer}").as_str())
                    .trim()
                    .parse()
                    .expect(format!("expected two numbers, got {buffer}").as_str());

                client
                    .post(game_url.as_str())
                    .json(&(row, column))
                    .send()
                    .await
                    .expect("failed to send the move");

                let game: Game = client
                    .get(game_url.as_str())
                    .send()
                    .await
                    .expect("failed to request game")
                    .json()
                    .await
                    .expect("failed to get request body");

                println!("{}", game.board());
            }
            State::Win(winner) => {
                println!("{winner} wins!");
                return;
            }
            State::Tie => {
                println!("Tie.");
                return;
            }
        }
    }
}
