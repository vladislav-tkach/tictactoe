use tictactoe::game::{self, Game};

use std::{io::stdin, str::FromStr};

struct Move(usize);

impl Move {
    fn column(&self) -> usize {
        return self.0 % 3;
    }

    fn row(&self) -> usize {
        return self.0 / 3;
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let Some(first_char) = trimmed.chars().next() else {
            return Err(format!(
                "Expected a number from 1 to 9, got empty string \"{s}\""
            ));
        };
        let Some(digit) = first_char.to_digit(10) else {
            return Err(format!(
                "Expected a number from 1 to 9, got not a number \"{first_char}\""
            ));
        };
        Ok(match digit {
            position @ 1..=9 => Move((position - 1) as usize),
            _ => {
                return Err(format!(
                    "Expected a number from 1 to 9, got number out of bounds \"{digit}\""
                ))
            }
        })
    }
}

fn main() {
    let mut game = Game::default();

    println!("{}", game.board());

    loop {
        let mut buffer = String::default();
        stdin()
            .read_line(&mut buffer)
            .expect("Couldn't read a line");

        let r#move: Move = buffer.parse().expect("Couldnt recognize the move");

        let state = game.make_turn(r#move.row(), r#move.column()).unwrap();

        println!("{}", game.board());

        match state {
            game::State::Playing => {}
            game::State::Win(mark) => {
                println!("{mark} wins!!!");
                break;
            }
            game::State::Tie => {
                println!("Tie!!!");
                break;
            }
        }
    }
}
