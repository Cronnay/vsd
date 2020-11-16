use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "v", long = "velocity", name = "Velocity in m/s")]
    velocity: Option<usize>,
    #[structopt(short = "d", long = "distance", name = "Distance in meters")]
    distance: Option<usize>,
    #[structopt(short = "t", long = "time", name = "Time in seconds")]
    time: Option<usize>,
}

fn main() -> CliResult {
    let args: Cli = Cli::from_args();
    match args {
        Cli {
            velocity: Some(v),
            distance: Some(d),
            time: _,
        } => println!("Calculating time: {}s", d / v),
        Cli {
            velocity: Some(v),
            distance: _,
            time: Some(t),
        } => println!("Calculating distance: {}m", v * t),
        Cli {
            velocity: _,
            distance: Some(d),
            time: Some(t),
        } => println!("Calculating velocity: {}m/s", d / t),
        Cli {
            velocity: _,
            distance: _,
            time: _,
        } => println!("Not valid format"),
    }

    Ok(())
}
