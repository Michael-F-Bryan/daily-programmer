use core::Difficulty;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, PartialEq)]
pub struct Args {
    #[structopt(
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        help = "Generate more verbose output"
    )]
    pub verbosity: u32,
    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(StructOpt, Debug, Clone, PartialEq)]
pub enum SubCommand {
    #[structopt(name = "list", about = "List all known challenges")]
    List,
    #[structopt(name = "run", about = "Execute a challenge")]
    Run {
        #[structopt(short = "d", long = "difficulty", default_value = "easy")]
        difficulty: Difficulty,
        #[structopt(short = "n", long = "number")]
        number: u32,
    },
    #[structopt(name = "run-all", about = "Run all the challenges")]
    RunAll,
}
