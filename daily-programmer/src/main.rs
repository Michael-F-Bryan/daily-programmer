mod cmd;
mod list;
mod run;

pub use crate::cmd::{Args, SubCommand};
pub use crate::list::list;
pub use crate::run::{run_all, run_one};

use core::Challenge;
use easy_370::Easy370;
use easy_371::Easy371;
use easy_374::Easy374;
use easy_375::Easy375;
use failure::Error;
use intermediate_374::Intermediate374;
use intermediate_375::Intermediate375;
use slog::{Drain, Level, Logger};
use structopt::StructOpt;

pub fn main() -> Result<(), Error> {
    let args = Args::from_args();
    let logger = initialize_logging(args.verbosity);

    slog::trace!(logger, "Parsed command-line arguments";
        "args" => format_args!("{:#?}", args));

    let challenges = all_challenges();

    match args.cmd {
        SubCommand::List => list(&challenges, args.verbosity),
        SubCommand::Run { difficulty, number } => {
            run_one(difficulty, number, &challenges, &logger)
        }
        SubCommand::RunAll => run_all(&challenges, &logger),
    }
}

pub fn all_challenges() -> Vec<Box<dyn Challenge>> {
    vec![
        Box::new(Easy370::default()),
        Box::new(Easy371::default()),
        Box::new(Easy374::default()),
        Box::new(Easy375::default()),
        Box::new(Intermediate374::default()),
        Box::new(Intermediate375::default()),
    ]
}

pub fn initialize_logging(verbosity: u32) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let level = match verbosity {
        0 => Level::Info,
        1 => Level::Debug,
        _ => Level::Trace,
    };
    let drain = drain.filter_level(level).fuse();

    Logger::root(drain, slog::o!())
}
