// re-export for convenience.
pub extern crate slog;
pub use failure::Error;
pub use slog::Logger;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

pub trait Challenge {
    /// Information describing this challenge.
    fn info(&self) -> &Info;
    /// Run the challenge to completion.
    fn execute(&self, logger: &Logger) -> Result<(), Error>;
}

/// Extra information about a challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    /// The challenge's title (80 characters or less).
    pub title: Cow<'static, str>,
    /// An extended description of the challenge.
    pub description: Cow<'static, str>,
    /// The challenge's difficulty.
    pub difficulty: Difficulty,
    /// The challenge number.
    pub number: u32,
    /// A URL which may be used to find the original challenge post.
    pub link: Cow<'static, str>,
}

/// How hard a particular [`Challenge`] is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum Difficulty {
    Easy,
    Intermediate,
    Hard,
}

impl FromStr for Difficulty {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "easy" => Ok(Difficulty::Easy),
            "intermediate" | "medium" => Ok(Difficulty::Intermediate),
            "hard" => Ok(Difficulty::Hard),
            other => {
                Err(failure::format_err!("Unknown difficulty, \"{}\"", other))
            }
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "easy"),
            Difficulty::Intermediate => write!(f, "intermediate"),
            Difficulty::Hard => write!(f, "hard"),
        }
    }
}

pub fn check_equal<T>(expected: T, got: T) -> Result<(), Error>
where
    T: PartialEq + Debug + Send + Sync + 'static,
{
    if expected == got {
        Ok(())
    } else {
        Err(Error::from(ExpectedEqual { expected, got }))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, failure_derive::Fail)]
struct ExpectedEqual<T>
where
    T: Debug + Send + Sync + 'static,
{
    expected: T,
    got: T,
}

impl<T> Display for ExpectedEqual<T>
where
    T: Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Expected {:?} but found {:?}", self.expected, self.got)
    }
}
