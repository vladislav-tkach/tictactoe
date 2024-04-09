use crate::mark::Mark;

use std::fmt::Display;

const BOARD_SIZE: usize = 3;

#[derive(Default)]
pub struct Board([[Option<Mark>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn mark(&mut self, row: usize, column: usize, mark: Mark) {
        self.0[row][column] = Some(mark);
    }

    pub fn at(&self, row: usize, column: usize) -> Option<Mark> {
        self.0[row][column]
    }

    pub fn full(&self) -> bool {
        self.0
            .iter()
            .flatten()
            .all(|maybe_mark| maybe_mark.is_some())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ┌─┬─┬─┐
        write!(f, "┌─")?;
        for _ in 0..(BOARD_SIZE - 1) {
            write!(f, "┬─")?;
        }
        writeln!(f, "┐")?;

        // │○│☓│ │
        for column in 0..BOARD_SIZE {
            match self.at(0, column) {
                Some(mark) => write!(f, "│{mark}")?,
                None => write!(f, "│ ")?,
            }
        }
        writeln!(f, "│")?;

        // ├─┼─┼─┤
        // │○│☓│ │
        for row in 1..BOARD_SIZE {
            // ├─┼─┼─┤
            write!(f, "├─")?;
            for _ in 0..(BOARD_SIZE - 1) {
                write!(f, "┼─")?;
            }
            writeln!(f, "┤")?;

            // │○│☓│ │
            for column in 0..BOARD_SIZE {
                match self.at(row, column) {
                    Some(mark) => write!(f, "│{mark}")?,
                    None => write!(f, "│ ")?,
                }
            }
            writeln!(f, "│")?;
        }

        // └─┴─┴─┘
        write!(f, "└─")?;
        for _ in 0..(BOARD_SIZE - 1) {
            write!(f, "┴─")?;
        }
        writeln!(f, "┘")
    }
}
