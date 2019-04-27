use core::{failure, Challenge, Difficulty, Error, Info, Logger};
use std::str::FromStr;

const TITLE: &str = "UPC check digits";
const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/a72sdj/20181217_challenge_370_easy_upc_check_digits/?utm_source=share&utm_medium=web2x";
const DESCRIPTION: &str = "# Description

The Universal Product Code (UPC-A) is a bar code used in many parts of the world. The bars encode a 12-digit number used to identify a product for sale, for example:

```
042100005264
```

The 12th digit (4 in this case) is a redundant check digit, used to catch errors. Using some simple calculations, a scanner can determine, given the first 11 digits, what the check digit must be for a valid code. (Check digits have previously appeared in this subreddit: see [Intermediate 30] and [Easy 197].) UPC's check digit is calculated as follows (taken from Wikipedia):

1. Sum the digits at odd-numbered positions (1st, 3rd, 5th, ..., 11th). *If you use 0-based indexing, this is the even-numbered positions (0th, 2nd, 4th, ... 10th)*.
2. Multiply the result from step 1 by 3.
3. Take the sum of digits at even-numbered positions (2nd, 4th, 6th, ..., 10th) in the original number, and add this sum to the result from step 2.
4. Find the result from step 3 modulo 10 (i.e. the remainder, when divided by 10) and call it M.
5. If *M* is 0, then the check digit is 0; otherwise the check digit is 10 - M.

For example, given the first 11 digits of a UPC `03600029145`, you can compute the check digit like this:

1. Sum the odd-numbered digits (0 + 6 + 0 + 2 + 1 + 5 = 14).
2. Multiply the result by 3 (14 × 3 = 42).
3. Add the even-numbered digits (42 + (3 + 0 + 0 + 9 + 4) = 58).
4. Find the result modulo 10 (58 divided by 10 is 5 remainder 8, so M = 8).
5. If *M* is not 0, subtract M from 10 to get the check digit (10 - M = 10 - 8 = 2).

So the check digit is `2`, and the complete UPC is `036000291452`.

# Challenge

Given an 11-digit number, find the 12th digit that would make a valid UPC. You may treat the input as a string if you prefer, whatever is more convenient. If you treat it as a number, you may need to consider the case of leading 0's to get up to 11 digits. That is, an input of `12345` would correspond to a UPC start of `00000012345`.

# Examples

```
upc(4210000526) => 4
upc(3600029145) => 2
upc(12345678910) => 4
upc(1234567) => 0
```

Also, if you live in a country that uses UPCs, you can generate all the examples you want by picking up store-bought items or packages around your house. Find anything with a bar code on it: if it has 12 digits, it's probably a UPC. Enter the first 11 digits into your program and see if you get the 12th.

[Intermediate 30]: https://www.reddit.com/r/dailyprogrammer/comments/red6f/3262012_challenge_30_intermediate/
[Easy 197]: https://www.reddit.com/r/dailyprogrammer/comments/2s7ezp/20150112_challenge_197_easy_isbn_validator/
";

#[derive(Debug, Clone, PartialEq)]
pub struct Easy370 {
    info: Info,
}

impl Default for Easy370 {
    fn default() -> Easy370 {
        Easy370 {
            info: Info {
                title: TITLE.into(),
                link: LINK.into(),
                description: DESCRIPTION.into(),
                difficulty: Difficulty::Easy,
                number: 370,
            },
        }
    }
}

impl Challenge for Easy370 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, _logger: &Logger) -> Result<(), Error> {
        Err(failure::err_msg("TODO: Implement this"))
    }
}

pub fn upc_check_digit(digits: &str) -> Result<u8, Error> {
    let d = Digits::from_str(digits)?;
    Ok(d.calculate_check_digit())
}

pub struct Digits([u8; 11]);

impl Digits {
    pub fn new(digits: [u8; 11]) -> Digits {
        assert!(
            digits.iter().all(|digit| *digit < 10),
            "All digits must be less than 10"
        );

        Digits(digits)
    }

    pub fn calculate_check_digit(&self) -> u8 {
        let sum = self.odd_sum() * 3 + self.even_sum();
        let m = sum % 10;

        if m == 0 {
            0
        } else {
            10 - m
        }
    }

    fn even_sum(&self) -> u8 {
        let mut even_sum = 0;

        let mut i = 1;
        while i < self.0.len() {
            even_sum += self.0[i];
            i += 2;
        }

        even_sum
    }

    fn odd_sum(&self) -> u8 {
        let mut odd_sum = 0;

        let mut i = 0;
        while i < self.0.len() {
            odd_sum += self.0[i];
            i += 2;
        }

        odd_sum
    }
}

impl FromStr for Digits {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buffer = [0; 11];
        let mut i = 0;

        for (ix, c) in s.char_indices() {
            if i >= buffer.len() {
                return Err(failure::err_msg(
                    "The input string should be exactly 11 characters",
                ));
            }

            match c.to_digit(10) {
                Some(digit) => buffer[i] = digit as u8,
                None => {
                    return Err(failure::format_err!(
                        "Expected a digit at index {}, but found {}",
                        ix,
                        c
                    ))
                }
            }

            i += 1;
        }

        if i != buffer.len() {
            return Err(failure::err_msg(
                "The input string should be exactly 11 characters",
            ));
        }

        Ok(Digits::new(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_inputs() {
        let inputs = vec![("04210000526", 4)];

        for (src, should_be) in inputs {
            let got = upc_check_digit(src).unwrap();
            assert_eq!(got, should_be);
        }
    }
}
