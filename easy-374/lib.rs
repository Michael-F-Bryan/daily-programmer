use core::{slog, Challenge, Difficulty, Error, Info, Logger};
use smallvec::SmallVec;
use std::iter::Sum;

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

    fn execute(&self, logger: &Logger) -> Result<(), Error> {
        let numbers: Vec<u64> = vec![
            13,
            1234,
            9876,
            0,
            10,
            19,
            199,
            1234567890,
            12345678901234567890,
        ];

        for number in numbers {
            let ap = additive_persistence(number);
            slog::info!(logger, ""; "number" => number, "additive-persistence" => ap);
        }

        Ok(())
    }
}

/// Calculate a number's *additive persistence*.
pub fn additive_persistence<N: Number>(n: N) -> u32 {
    let mut count = 0;
    let mut n = n;
    while n.two_or_more_digits() {
        n = sum_digits(n);
        count += 1;
    }

    count
}

fn sum_digits<N: Number>(n: N) -> N {
    digits(n).map(N::from_u8).sum()
}

/// A digit buffer.
pub type Buffer = SmallVec<[u8; 8]>;

/// Get an iterator over the digits in a number.
pub fn digits<N: Number>(n: N) -> impl Iterator<Item = u8> {
    let mut buffer = Buffer::new();
    n.digits_rec(&mut buffer);
    buffer.into_iter()
}

/// A generic number for the purposes of calculating *Additive Persistence*.
pub trait Number: Sum {
    /// Push the digits of this number onto the buffer.
    fn digits_rec(&self, buffer: &mut Buffer);
    /// Does this number have 2 or more digits?
    fn two_or_more_digits(&self) -> bool;
    fn from_u8(n: u8) -> Self;
}

macro_rules! impl_number {
    ($ty:ty) => {
        impl $crate::Number for $ty {
            #[allow(unused_comparisons)]
            fn digits_rec(&self, buffer: &mut Buffer) {
                debug_assert!(*self >= 0,
                    "It doesn't make sense to find the digits in a negative number");

                if *self >= 10 {
                    (*self/10).digits_rec(buffer);
                }

                let digit = *self % 10;
                buffer.push(digit as u8);
            }

            fn two_or_more_digits(&self) -> bool {
                *self >= 10
            }

            fn from_u8(n: u8) -> Self {
                n as $ty
            }
        }
    };
    ( $( $ty:ty ),*) => {
        $(
            impl_number!($ty);
        )*
    }
}

impl_number!(u8, u16, u32, u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_values() {
        let inputs = vec![(13_u32, 1), (1234, 2), (9876, 2), (199, 3)];

        for (input, should_be) in inputs {
            let got = additive_persistence(input);

            assert_eq!(got, should_be, "for {}", input);
        }
    }

    #[test]
    fn get_number_digits() {
        let inputs: Vec<(u32, &[u8])> = vec![
            (0, &[0]),
            (10, &[1, 0]),
            (111, &[1, 1, 1]),
            (101010, &[1, 0, 1, 0, 1, 0]),
        ];

        for (input, should_be) in inputs {
            let got: Vec<_> = digits(input).collect();

            assert_eq!(got, should_be);
        }
    }
}
