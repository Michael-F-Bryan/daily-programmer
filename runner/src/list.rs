use core::Challenge;
use failure::Error;

pub fn list(
    challenges: &[Box<dyn Challenge>],
    verbosity: u32,
) -> Result<(), Error> {
    let more_info = verbosity > 0;

    if more_info {
        verbose_list(challenges);
    } else {
        brief_list(challenges);
    }

    Ok(())
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

        println!();
    }
}

fn brief_list(challenges: &[Box<dyn Challenge>]) {
    for challenge in challenges {
        let info = challenge.info();
        println!("{}-{}\t{}", info.difficulty, info.number, info.title);
    }
}
