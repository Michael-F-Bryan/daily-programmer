use core::{Challenge, Difficulty, Error, Info, Logger};

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

    fn execute(&self, _logger: &Logger) -> Result<(), Error> {
        unimplemented!()
    }
}
