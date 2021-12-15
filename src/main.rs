use clap::App;
use clap::Arg;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;

mod crate_info;
mod days;

use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;

pub type Solution = (String, String);

fn main() -> Result<(), std::io::Error> {
    let cli = App::new(crate_name())
        .version(crate_version())
        .about(crate_description())
        .author(crate_author())
        .arg(
            Arg::with_name("day")
                .takes_value(true)
                .short("d")
                .long("day")
                .help(r#"Day number (1 - 25) to run. If omitted, no days are run."#),
        );

    let matches = cli.get_matches();

    if let Some(day) = matches.value_of("day") {
        run_day(
            day.parse::<u8>()
                .unwrap_or_else(|_| panic!("Invalid day number: {}", day)),
        )
    } else {
        Ok(())
    }
}

fn day_input_filename(day: u8) -> PathBuf {
    let padded_day = format!("{:02}", day);
    Path::new("inputs").join(format!("day{}.in", padded_day))
}

fn run_day(day: u8) -> Result<(), std::io::Error> {
    println!();
    println!("=== Day {: >2} ===", day);

    let day_func = days::get_solver(day).unwrap_or_else(|| panic!("Unknown day: {}", day));
    let input = read_to_string(&day_input_filename(day))?;
    day_func(&input);

    Ok(())
}
