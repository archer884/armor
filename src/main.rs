use clap::{Parser, Subcommand};

/// armor calculator
#[derive(Debug, Parser)]
#[clap(about, author, version)]
struct Args {
    // For now, this really isn't optional, because I haven't thought of anything else for the
    // program to do.
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Calculates the line-of-sight thickness of a plate sloped at N° from the vertical. (Aliases
    /// for this command inclode l/los)
    #[clap(alias = "l", alias = "los")]
    LineOfSight(LineOfSight),
}

#[derive(Debug, Parser)]
struct LineOfSight {
    /// normal thickness
    normal: f64,

    /// slope in degrees (0° == vertical)
    angle: f64,

    /// secondary (lateral) angle in degrees (optional; 0° == facing head on)
    secondary: Option<f64>,
}

fn main() {
    run(&Args::parse());
}

fn run(args: &Args) {
    match &args.command {
        Command::LineOfSight(los) => calculate_line_of_sight(los),
    }
}

/// Calculate the line-of-sight thickness of a plate at a given slope
fn calculate_line_of_sight(args: &LineOfSight) {
    let angle = args
        .secondary
        .map(|lateral| compound_angle(args.angle.to_radians(), lateral.to_radians()))
        .unwrap_or_else(|| args.angle.to_radians());

    let line_of_sight = args.normal / angle.cos();
    println!("{line_of_sight:.02}");
}

/// Combines two angles
///
/// This function combines, for instance, the angle of a sloped plate with the angle of a vehicle
/// not directly facing the viewer, as with a Tiger in an optimal fighting position.
///
/// This function operates in RADIANS. It is a mathematical error to call this function with values
/// representing DEGREES.
fn compound_angle(a: f64, b: f64) -> f64 {
    (a.cos() * b.cos()).acos()
}
