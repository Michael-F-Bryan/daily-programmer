use core::{slog, Challenge, Difficulty, Error, Info, Logger};
use std::iter::FromIterator;

pub const TITLE: &str = "The Game of Blobs";
pub const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/aldexk/20190130_challenge_374_intermediate_the_game_of/?utm_source=share&utm_medium=web2x";
pub const DESCRIPTION: &str = "# Description

You are give a list of blobs, each having an initial position in an discrete grid, and a size. Blobs try to eat each other greedily and move around accordingly.

During each cycle, all blobs move one step (Moore neighborhood) towards another blob of smaller size (if any). This blob is chosen as the closest one, with a preference for larger ones, breaking ties as clockwise (11H < 12H > 01H).

At the end of each cycle, blobs merge (with summed size) if they are on the same location.

Return the final state of the blobs.

# Example:

Given: `[(0,2,1),(2,1,2)]` as a list of (x,y and size)

```
..1    ..1    ..3
...    ..2    ...
.2.    ...    ...
```

Solution: `[(0,2)]`

# Challenge

```
[(0,1,2),
 (10,0,2)]

[(4, 3, 4), 
 (4, 6, 2), 
 (8, 3, 2), 
 (2, 1, 3)]

[(-57, -16, 10),
 (-171, -158, 13),
 (-84, 245, 15),
 (-128, -61, 16),
 (65, 196, 4),
 (-221, 121, 8),
 (145, 157, 3),
 (-27, -75, 5)]
```

# Bonus

Help the blobs break out of flatland.

Given: `[(1,2),(4,2)]`

```
.1..2    .1.2.    .12..    .3...
```

A solution: `[(1,3)]`

Given `[(0,2,0,1),(1,2,1,2)]`

```
..1    .21    ..3
...    ...    ...
/      /      /
...    ...    ...
2..    ...    ...
```

A solution `[(0,2,0)]`

# Bonus 2

Mind that the distances can be long. Try to limit run times.

# Bonus Challenges

```
[(6,3), 
 (-7,4), 
 (8,3), 
 (7,1)]

[(-7,-16,-16,4),
 (14,11,12,1),
 (7,-13,-13,4),
 (-9,-8,-11,3)]
.

[(-289429971, 243255720, 2),
 (2368968216, -4279093341, 3),
 (-2257551910, -3522058348, 2),
 (2873561846, -1004639306, 3)]
```

# Credits

This challenge was suggested by /user/tomekanco, many thanks! Have a good challenge idea? Consider submitting it to /r/dailyprogrammer_ideas and there's a good chance we'll use it.";

#[derive(Debug, Clone, PartialEq)]
pub struct Intermediate374 {
    info: Info,
}

impl Default for Intermediate374 {
    fn default() -> Intermediate374 {
        Intermediate374 {
            info: Info {
                title: TITLE.into(),
                link: LINK.into(),
                description: DESCRIPTION.into(),
                difficulty: Difficulty::Intermediate,
                number: 374,
            },
        }
    }
}

impl Challenge for Intermediate374 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, logger: &Logger) -> Result<(), Error> {
        slog::error!(logger, "TODO: Implement this");
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blob {
    pub x: i32,
    pub y: i32,
    pub size: u32,
}

impl Blob {
    pub fn new(x: i32, y: i32, size: u32) -> Blob {
        Blob { x, y, size }
    }

    pub fn distance_to(&self, x: i32, y: i32) -> u32 {
        let dx = x - self.x;
        let dy = y - self.y;
        let manhattan_distance = dx.abs() + dy.abs();

        manhattan_distance as u32
    }
}

fn move_blob_towards(blob: &mut Blob, target: &Blob) {
    let (x, y) = (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|(dx, dy)| *dx != 0 && *dy != 0)
        .map(|(dx, dy)| (blob.x + dx, blob.y + dy))
        .min_by_key(|(x, y)| target.distance_to(*x, *y))
        .unwrap();

    blob.x = x;
    blob.y = y;
}

/// The game board.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    blobs: Vec<Blob>,
}

impl Board {
    pub fn new(blobs: Vec<Blob>) -> Board {
        let mut b = Board { blobs };
        b.normalize();
        b
    }

    pub fn blobs(&self) -> &[Blob] {
        &self.blobs
    }

    pub fn step(&mut self) {
        self.move_blobs();
        self.merge_blobs();
    }

    pub fn is_finished(&self) -> bool {
        self.blobs.len() <= 1
    }

    fn move_blobs(&mut self) {
        for i in 0..self.blobs.len() {
            if let Some(target) = self.closest_smaller_blob_to(self.blobs[i]) {
                move_blob_towards(&mut self.blobs[i], &target);
            }
        }
    }

    fn closest_smaller_blob_to(&self, blob: Blob) -> Option<Blob> {
        self.blobs
            .iter()
            .filter(|b| b.size < blob.size)
            .min_by_key(|b| blob.distance_to(b.x, b.y))
            .cloned()
    }

    fn merge_blobs(&mut self) {
        self.blobs.sort_by_key(|b| (b.x, b.y));
        let mut new_buffer = Vec::<Blob>::with_capacity(self.blobs.capacity());

        for blob in &self.blobs {
            if let Some(last_blob) = new_buffer.last_mut() {
                if last_blob.x == blob.x && last_blob.y == blob.y {
                    last_blob.size += blob.size;
                    continue;
                }
            }

            new_buffer.push(*blob);
        }

        self.blobs = new_buffer;
    }

    /// Sort the blobs by coordinate so blobs on the same tile will be next to
    /// each other.
    fn normalize(&mut self) {
        self.blobs.sort_by_key(|blob| (blob.x, blob.y));
    }
}

impl FromIterator<Blob> for Board {
    fn from_iter<I: IntoIterator<Item = Blob>>(iter: I) -> Board {
        Board::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn step_through_a_game() {
        // ```
        // t = 0          t = 1            t = 2
        // ..1             ..1              ..3
        // ...             ..2              ...
        // .2.             ...              ...
        // ```
        let blobs = vec![Blob::new(0, 2, 1), Blob::new(2, 1, 2)];
        let mut board = Board::new(blobs);

        board.step();

        let new_positions = &[Blob::new(0, 2, 1), Blob::new(1, 2, 2)];
        assert_eq!(board.blobs(), new_positions);

        board.step();

        let new_positions = &[Blob::new(0, 2, 3)];
        assert_eq!(board.blobs(), new_positions);

        assert!(board.is_finished());
    }

    #[test]
    fn merge_two_blobs() {
        let blobs = vec![Blob::new(0, 0, 1), Blob::new(0, 0, 2)];
        let mut board = Board::new(blobs);

        board.merge_blobs();

        let should_be = vec![Blob::new(0, 0, 3)];
        assert_eq!(should_be, board.blobs);
    }
}
