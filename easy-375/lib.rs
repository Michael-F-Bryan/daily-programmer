use core::{Challenge, Difficulty, Info};
use failure::Error;
use slog::Logger;

pub const TITLE: &str = "Print a new number by adding one to each of its digit";
pub const LINK: &str = "https://www.reddit.com/r/dailyprogrammer/comments/aphavc/20190211_challenge_375_easy_print_a_new_number_by/?utm_source=share&utm_medium=web2x";
pub const DESCRIPTION: &str = "# Description

A number is input in computer then a new no should get printed by adding one to each of its digit. If you encounter a 9, insert a 10 (don't carry over, just shift things around).

For example, 998 becomes 10109.

# Bonus

This challenge is trivial to do if you map it to a string to iterate over the input, operate, and then cast it back. Instead, try doing it without casting it as a string at any point, keep it numeric (int, float if you need it) only.

# Credit

This challenge was suggested by user /u/chetvishal, many thanks! If you have a challenge idea please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it.";

#[derive(Debug, Clone, PartialEq)]
pub struct Easy375 {
    info: Info,
}

impl Default for Easy375 {
    fn default() -> Easy375 {
        Easy375 {
            info: Info {
                title: String::from(TITLE),
                difficulty: Difficulty::Easy,
                number: 375,
                link: String::from(LINK),
                description: String::from(DESCRIPTION),
            },
        }
    }
}

impl Challenge for Easy375 {
    fn info(&self) -> &Info {
        &self.info
    }

    fn execute(&self, logger: &Logger) -> Result<(), Error> {
        let input = 998;
        let expected = 10109;

        slog::info!(logger, "Running the example"; 
            "input" => input, 
            "expected" => expected);

        let got = easy_375(input);

        core::check_equal(expected, got)
    }
}

pub fn easy_375(n: u64) -> u64 {
    unimplemented!()
}
