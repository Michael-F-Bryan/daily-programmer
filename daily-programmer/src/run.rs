use core::{Challenge, Difficulty};
use failure::Error;
use slog::Logger;
use std::time::Instant;

pub fn run_one(
    difficulty: Difficulty,
    number: u32,
    challenges: &[Box<dyn Challenge>],
    logger: &Logger,
) -> Result<(), Error> {
    let challenge = challenges
        .into_iter()
        .find(|challenge| {
            let info = challenge.info();
            info.number == number && info.difficulty == difficulty
        })
        .ok_or_else(|| {
            failure::format_err!(
                "No known challenge with difficulty of {} and number {}",
                difficulty,
                number
            )
        })?;

    run(&**challenge, logger)
}

pub fn run_all(
    challenges: &[Box<dyn Challenge>],
    logger: &Logger,
) -> Result<(), Error> {
    let mut errors = Vec::new();

    for challenge in challenges {
        let info = challenge.info();

        if let Err(e) = run(&**challenge, logger) {
            errors.push((info.clone(), e));
        }
    }

    if errors.len() == 0 {
        Ok(())
    } else {
        unimplemented!("TODO: find a way to report all errors")
    }
}

fn run(challenge: &dyn Challenge, logger: &Logger) -> Result<(), Error> {
    let info = challenge.info();
    let logger = logger.new(slog::o!("title" => info.title.to_string()));
    slog::info!(logger, "Starting the challenge");

    let start = Instant::now();
    let result = challenge.execute(&logger);
    let duration = Instant::now() - start;

    slog::info!(logger, "Finished running the challenge"; 
        "duration" => format_args!("{:?}", duration));

    result
}
