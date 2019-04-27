use core::{failure, slog, Challenge, Difficulty, Error, Info, Logger};
use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

pub const TITLE: &str = "A Card Flipping Game";
pub const DESCRIPTION: &str = r#"
# Description

This challenge is about [a simple card flipping solitaire game](https://www.youtube.com/watch?v=CCxs-tu8tOU). You're presented with a sequence of cards, some face up, some face down. You can remove any face up card, but you must then flip the adjacent cards (if any). The goal is to successfully remove every card. Making the wrong move can get you stuck.

In this challenge, a 1 signifies a face up card and a 0 signifies a face down card. We will also use zero-based indexing, starting from the left, to indicate specific cards. So, to illustrate a game, consider this starting card set.

```
0100110
```

I can choose to remove cards 1, 4, or 5 since these are face up. If I remove card 1, the game looks like this (using . to signify an empty spot):

```
1.10110
```

I had to flip cards 0 and 2 since they were adjacent. Next I could choose to remove cards 0, 2, 4, or 5. I choose card 0:

```
..10110
```

Since it has no adjacent cards, there were no cards to flip. I can win this game by continuing with: 2, 3, 5, 4, 6.

Supposed instead I started with card 4:

```
0101.00
```

This is unsolvable since there's an "island" of zeros, and cards in such islands can never be flipped face up.

# Input Description

As input you will be given a sequence of 0 and 1, no spaces.

# Output Description

Your program must print a sequence of moves that leads to a win. If there is no solution, it must print "no solution". In general, if there's one solution then there are many possible solutions.

Optional output format: Illustrate the solution step by step.

# Sample Inputs

```
0100110
01001100111
100001100101000
```

# Sample Outputs

```
1 0 2 3 5 4 6
no solution
0 1 2 3 4 6 5 7 8 11 10 9 12 13 14
```

# Challenge Inputs

```
0100110
001011011101001001000
1010010101001011011001011101111
1101110110000001010111011100110
```

# Bonus Input

```
010111111111100100101000100110111000101111001001011011000011000
```

# Credit
This challenge was suggested by /u/skeeto, many thanks! If you have a challenge idea please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it."#;
pub const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/aq6gfy/20190213_challenge_375_intermediate_a_card/?utm_source=share&utm_medium=web2x";

#[derive(Debug, Clone, PartialEq)]
pub struct Intermediate375 {
    info: Info,
}

impl Default for Intermediate375 {
    fn default() -> Intermediate375 {
        Intermediate375 {
            info: Info {
                title: TITLE.into(),
                difficulty: Difficulty::Intermediate,
                number: 375,
                link: LINK.into(),
                description: DESCRIPTION.into(),
            },
        }
    }
}

impl Challenge for Intermediate375 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, logger: &Logger) -> Result<(), Error> {
        let inputs = [
            "0100110",
            "01001100111",
            "100001100101000",
            "0100110",
            "001011011101001001000",
            "1010010101001011011001011101111",
            "1101110110000001010111011100110",
        ];

        for input in &inputs[..] {
            let game = Game::from_str(input)?;
            match solve(game.cards()) {
                Some(solution) => {
                    let words = solution
                        .into_iter()
                        .map(|r| r.0.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    slog::info!(logger, "Solved"; 
                    "input" => input,
                    "solution" => words);
                }
                None => slog::info!(logger, "No solution"; "input" => input),
            }
        }

        Ok(())
    }
}

/// The possible game states.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    FaceUp,
    FaceDown,
    Removed,
}

impl Card {
    pub fn flipped(self) -> Card {
        match self {
            Card::FaceDown => Card::FaceUp,
            Card::FaceUp => Card::FaceDown,
            Card::Removed => Card::Removed,
        }
    }
}

/// Remove whatever is at the specified index.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Remove(pub usize);

/// The current state of the game.
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    state: Vec<Card>,
}

impl Game {
    pub fn new(cards: Vec<Card>) -> Game {
        Game { state: cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.state
    }

    pub fn is_won(&self) -> bool {
        self.state.iter().all(|card| *card == Card::Removed)
    }

    pub fn execute(&mut self, cmd: Remove) -> Result<(), InvalidMove> {
        let index = cmd.0;

        match self.state.get(cmd.0).cloned() {
            Some(Card::FaceUp) => {
                self.state[index] = Card::Removed;
                if let Some(l) = self.left_of(index) {
                    *l = l.flipped();
                    println!("Left of {} was flipped to {:?}", index, *l);
                }
                if let Some(r) = self.right_of(index) {
                    *r = r.flipped();
                    println!("Right of {} was flipped to {:?}", index, *r);
                }
                Ok(())
            }
            Some(Card::FaceDown) => Err(InvalidMove::NotFacingUp { index }),
            Some(Card::Removed) => Err(InvalidMove::AlreadyRemoved { index }),
            None => Err(InvalidMove::IndexOutOfBounds {
                index,
                length: self.state.len(),
            }),
        }
    }

    fn left_of(&mut self, ix: usize) -> Option<&mut Card> {
        if 0 < ix
            && ix < self.state.len()
            && self.state[ix - 1] != Card::Removed
        {
            Some(&mut self.state[ix - 1])
        } else {
            None
        }
    }

    fn right_of(&mut self, ix: usize) -> Option<&mut Card> {
        if ix + 1 < self.state.len() && self.state[ix + 1] != Card::Removed {
            Some(&mut self.state[ix + 1])
        } else {
            None
        }
    }
}

impl FromIterator<Card> for Game {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Game {
        Game::new(iter.into_iter().collect())
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();

        for (i, digit) in s.char_indices() {
            match digit {
                '0' => digits.push(Card::FaceDown),
                '1' => digits.push(Card::FaceUp),
                '.' => digits.push(Card::Removed),
                _ => {
                    return Err(failure::format_err!(
                        "Expected either 1 or 0 at index {}, but found {:?}",
                        i,
                        digit
                    ))
                }
            }
        }

        Ok(Game::new(digits))
    }
}

pub fn solve(initial_state: &[Card]) -> Option<Vec<Remove>> {
    let mut commands = VecDeque::new();
    let mut next_card_higher = false;

    for (i, card) in initial_state.into_iter().cloned().enumerate() {
        if next_card_higher {
            commands.push_back(Remove(i));
        } else {
            commands.push_front(Remove(i));
        }

        next_card_higher ^= card == Card::FaceUp;
    }

    if next_card_higher {
        Some(commands.into())
    } else {
        None
    }
}

/// An error due to an invalid move.
#[derive(Debug, Copy, Clone, PartialEq, failure_derive::Fail)]
pub enum InvalidMove {
    IndexOutOfBounds { index: usize, length: usize },
    NotFacingUp { index: usize },
    AlreadyRemoved { index: usize },
}

impl Display for InvalidMove {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InvalidMove::IndexOutOfBounds { index, length } => write!(
                f,
                "index out of bounds: there are {} cards but the index was {}",
                length, index
            ),
            InvalidMove::NotFacingUp { index } => {
                write!(f, "You can't remove the card at index {}", index)
            }
            InvalidMove::AlreadyRemoved { index } => {
                write!(f, "The card at index {} is already removed", index)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_string() {
        let should_be = Game::new(vec![
            Card::FaceDown,
            Card::FaceUp,
            Card::FaceDown,
            Card::FaceDown,
            Card::FaceUp,
            Card::FaceUp,
            Card::FaceDown,
        ]);
        let src = "0100110";

        let got = Game::from_str(src).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn run_the_example() {
        let initial_state = "0100110";
        let mut subsequent_states = vec![
            "1.10110", "..10110", "...1110", "....010", "....1.1", "......1",
            ".......",
        ];
        let moves = vec![
            Remove(1),
            Remove(0),
            Remove(2),
            Remove(3),
            Remove(5),
            Remove(4),
            Remove(6),
        ];

        let mut game = Game::from_str(initial_state).unwrap();

        for (i, m) in moves.into_iter().enumerate() {
            println!("Move {} = {:?}", i, m);
            game.execute(m).unwrap();

            let should_be = Game::from_str(&subsequent_states[0]).unwrap();
            assert_eq!(game, should_be);
            subsequent_states.remove(0);
        }

        assert!(game.is_won());
    }

    #[test]
    fn solve_the_example() {
        let initial_state = "0100110";
        let mut game = Game::from_str(initial_state).unwrap();

        let solution = solve(game.cards()).unwrap();

        for step in solution {
            game.execute(step).unwrap();
        }

        assert!(game.is_won());
    }
}
