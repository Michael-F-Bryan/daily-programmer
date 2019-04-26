use core::{slog, Challenge, Difficulty, Error, Info, Logger};
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

const TITLE: &str = "N queens validator";
const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/ab9mn7/20181231_challenge_371_easy_n_queens_validator/?utm_source=share&utm_medium=web2x";
const DESCRIPTION: &str = "# Description

For the purpose of this challenge, the N queens problem consists of putting one queen on every column (labeled a, b, c, ...) of an NxN chessboard, such that no two queens are in the same row or diagonal. An example valid solution for `N = 6` is:

```
6  . . Q . . .
5  . . . . . Q
4  . Q . . . .
3  . . . . Q .
2  Q . . . . .
1  . . . Q . .
   a b c d e f
```

In chess notation, the squares with queens in this solution are called `a2`, `b4`, `c6`, `d1`, `e3`, and `f5`. We'll represent solutions by listing the rows that each column's queen appears in from left to right, so this solution is represented as the array `{2, 4, 6, 1, 3, 5}`.

Solving the N queens problem was [#25][hard-25] (difficult) on `r/dailyprogrammer`, but you don't need to actually solve it for today's challenge.

# Challenge

Given an array of 8 integers between 1 and 8, determine whether it represents a valid 8 queens solution.

```
qcheck({4, 2, 7, 3, 6, 8, 5, 1}) => true
qcheck({2, 5, 7, 4, 1, 8, 6, 3}) => true
qcheck({5, 3, 1, 4, 2, 8, 6, 3}) => false   (b3 and h3 are on the same row)
qcheck({5, 8, 2, 4, 7, 1, 3, 6}) => false   (b8 and g3 are on the same diagonal)
qcheck({4, 3, 1, 8, 1, 3, 5, 2}) => false   (multiple problems)
```

You may optionally handle solutions for any N, not just `N = 8`.

# Optional bonus

In this bonus, you are given an invalid solution where it's possible to swap two numbers and produce a valid solution, which you must find. (Be aware that most invalid solutions will not have this property.)

For example, `{8, 6, 4, 2, 7, 1, 3, 5}` is invalid because `c4` and `f1` are on the same diagonal. But if you swap the 8 and the 4 (i.e. replace `a8` and `c4` with `a4` and `c8`), you get the valid solution `{4, 6, 8, 2, 7, 1, 3, 5}`.

```
qfix({8, 6, 4, 2, 7, 1, 3, 5}) => {4, 6, 8, 2, 7, 1, 3, 5}
qfix({8, 5, 1, 3, 6, 2, 7, 4}) => {8, 4, 1, 3, 6, 2, 7, 5}
qfix({4, 6, 8, 3, 1, 2, 5, 7}) => {4, 6, 8, 3, 1, 7, 5, 2}
qfix({7, 1, 3, 6, 8, 5, 2, 4}) => {7, 3, 1, 6, 8, 5, 2, 4}
```

[hard-24]: https://www.reddit.com/r/dailyprogrammer/comments/qxv8h/3152012_challenge_25_difficult/
";

#[derive(Debug, Clone, PartialEq)]
pub struct Easy371 {
    info: Info,
}

impl Challenge for Easy371 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, logger: &Logger) -> Result<(), Error> {
        let inputs = [
            [4, 2, 7, 3, 6, 8, 5, 1],
            [2, 5, 7, 4, 1, 8, 6, 3],
            [5, 3, 1, 4, 2, 8, 6, 3],
            [5, 8, 2, 4, 7, 1, 3, 6],
            [4, 3, 1, 8, 1, 3, 5, 2],
        ];

        for input in &inputs {
            let got = qcheck(input);
            let tiles = to_tiles(input).join(" ");

            match got {
                Outcome::Valid => {
                    slog::info!(logger, "Solution is valid"; "solution" => &tiles)
                }
                Outcome::Conflict(left, right) => {
                    slog::info!(logger, "Solution has one or more conflicts";
                        "first-queen" => left.to_string(),
                        "second-queen" => right.to_string());
                }
            }
        }

        Ok(())
    }
}

impl Default for Easy371 {
    fn default() -> Easy371 {
        Easy371 {
            info: Info {
                title: TITLE.into(),
                description: DESCRIPTION.into(),
                link: LINK.into(),
                difficulty: Difficulty::Easy,
                number: 371,
            },
        }
    }
}

fn to_tiles(locations: &[usize]) -> impl Iterator<Item = Tile> + '_ {
    locations
        .into_iter()
        .cloned()
        .enumerate()
        .map(|(column, row)| Tile::new(row - 1, column))
}

pub fn qcheck(locations: &[usize]) -> Outcome {
    qcheck_with_conflict(locations, conflicts)
}

fn qcheck_with_conflict<F>(locations: &[usize], has_conflicts: F) -> Outcome
where
    F: Fn(Tile, Tile) -> bool,
{
    let queens: Vec<_> = to_tiles(locations).collect();

    for queen in &queens {
        let other_queens = queens.iter().filter(|q| *q != queen);
        for other_queen in other_queens {
            if has_conflicts(*queen, *other_queen) {
                return Outcome::Conflict(*queen, *other_queen);
            }
        }
    }

    Outcome::Valid
}

pub fn conflicts(a: Tile, b: Tile) -> bool {
    conflicts_horizontally(a, b)
        || conflicts_vertically(a, b)
        || conflicts_diagonally(a, b)
}

fn conflicts_vertically(a: Tile, b: Tile) -> bool {
    a.column == b.column
}

fn conflicts_horizontally(a: Tile, b: Tile) -> bool {
    a.row == b.row
}

fn conflicts_diagonally(a: Tile, b: Tile) -> bool {
    let delta_x = a.column as i32 - b.column as i32;
    let delta_y = a.row as i32 - b.row as i32;

    delta_x.abs() == delta_y.abs()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Outcome {
    Valid,
    Conflict(Tile, Tile),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile {
    pub row: usize,
    pub column: usize,
}

impl Tile {
    pub fn new(row: usize, column: usize) -> Tile {
        Tile { row, column }
    }

    pub fn row_letter(&self) -> char {
        let value = 'a' as usize + self.column;
        debug_assert!(value < u8::max_value() as usize);
        value as u8 as char
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.row_letter(), self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_solutions() {
        let inputs = vec![[4, 2, 7, 3, 6, 8, 5, 1], [2, 5, 7, 4, 1, 8, 6, 3]];

        for (i, input) in inputs.into_iter().enumerate() {
            let got = qcheck(&input);
            assert_eq!(got, Outcome::Valid, "at index {}", i);
        }
    }

    #[test]
    fn same_row() {
        let input = [5, 3, 1, 4, 2, 8, 6, 3];
        let should_be = Outcome::Conflict(Tile::new(2, 1), Tile::new(2, 7));

        let got = qcheck_with_conflict(&input, conflicts_horizontally);

        assert_eq!(got, should_be);
    }

    #[test]
    fn diagonals() {
        let input = [5, 8, 2, 4, 7, 1, 3, 6];
        let should_be = Outcome::Conflict(Tile::new(7, 1), Tile::new(2, 6));

        let got = qcheck_with_conflict(&input, conflicts_diagonally);

        assert_eq!(got, should_be);
    }

    #[test]
    fn many_issues() {
        let input = [4, 3, 1, 8, 1, 3, 5, 2];

        let got = qcheck(&input);

        assert_ne!(got, Outcome::Valid);
    }
}
