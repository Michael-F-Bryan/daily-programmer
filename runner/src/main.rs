use core::{Challenge, Difficulty};
use easy_375::Easy375;
use structopt::StructOpt;

pub fn main() {
    let args = Args::from_args();
    let challenges = all_challenges();

    match args.cmd {
        SubCommand::List => list(&challenges, args.verbosity),
        SubCommand::Run { .. } => unimplemented!(),
        SubCommand::RunAll => unimplemented!(),
    }
}

pub fn all_challenges() -> Vec<Box<dyn Challenge>> {
    vec![Box::new(Easy375::default())]
}

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

fn list(challenges: &[Box<dyn Challenge>], verbosity: u32) {
    let more_info = verbosity > 0;

    if more_info {
        verbose_list(challenges);
    } else {
        brief_list(challenges);
    }
}

fn verbose_list(challenges: &[Box<dyn Challenge>]) {
    let width = textwrap::termwidth();
    let indent = "  ";

    for challenge in challenges {
        let info = challenge.info();
        let line =
            format!("{}-{}: {}", info.difficulty, info.number, info.title);
        println!("{}", line);

        println!("{}", "-".repeat(line.len()));
        println!();

        for line in textwrap::wrap_iter(&info.description, width - indent.len())
        {
            println!("{}{}", indent, line);
        }
    }
}

fn brief_list(challenges: &[Box<dyn Challenge>]) {
    for challenge in challenges {
        let info = challenge.info();
        println!("{}-{}\t{}", info.difficulty, info.number, info.title);
    }
}
