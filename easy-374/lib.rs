use core::{Challenge, Difficulty, Error, Info, Logger};

pub const TITLE: &str = "Additive Persistence";
pub const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/akv6z4/20190128_challenge_374_easy_additive_persistence/?utm_source=share&utm_medium=web2x";
pub const DESCRIPTION: &str = "# Description

Inspired by [this tweet](https://twitter.com/fermatslibrary/status/1089883307473543170), today's challenge is to calculate the [*additive persistence*](http://mathworld.wolfram.com/AdditivePersistence.html) of a number, defined as how many loops you have to do summing its digits until you get a single digit number. Take an integer *N*:

1. Add its digits
2. Repeat until the result has 1 digit

The total number of iterations is the additive persistence of *N*.

Your challenge today is to implement a function that calculates the additive persistence of a number.

# Examples

```
13 -> 1
1234 -> 2
9876 -> 2
199 -> 3
```

# Bonus

The really easy solution manipulates the input to convert the number to a string and iterate over it. Try it without making the number a strong, decomposing it into digits while keeping it a number.

On some platforms and languages, if you try and find ever larger persistence values you'll quickly learn about your platform's big integer interfaces (e.g. 64 bit numbers).";

#[derive(Debug, Clone, PartialEq)]
pub struct Easy374 {
    info: Info,
}

impl Default for Easy374 {
    fn default() -> Easy374 {
        Easy374 {
            info: Info {
                title: TITLE.into(),
                link: LINK.into(),
                description: DESCRIPTION.into(),
                number: 374,
                difficulty: Difficulty::Easy,
            },
        }
    }
}

impl Challenge for Easy374 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, _logger: &Logger) -> Result<(), Error> {
        unimplemented!()
    }
}
