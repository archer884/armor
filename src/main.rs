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
    /// Calculates the effective thickness of a plate sloped at N° from the vertical. (Aliases for
    /// this command inclode l/los)
    #[clap(alias = "l", alias = "los")]
    LineOfSight(LineOfSight),

    /// Calculates the slope of armor from normal and effective thickness.
    #[clap(alias = "fs", alias = "slope")]
    FindSlope(FindSlope),

    /// Calculates the required normal thickness of a plate presenting a given effective thickness
    /// at a given angle of incidence.
    #[clap(alias = "fn", alias = "normal")]
    FindNormal(FindNormal),
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

impl LineOfSight {
    /// The angle of incidence in RADIANS
    ///
    /// If a secondary angle is supplied, this value includes the compound angle. See
    /// [`compound_angle`] for details.
    fn angle(&self) -> f64 {
        let a = self.angle.to_radians();
        let b = self.secondary.map(|secondary| secondary.to_radians());
        b.map(|b| compound_angle(a, b)).unwrap_or(a)
    }
}

#[derive(Debug, Parser)]
struct FindSlope {
    /// normal thickness
    normal: f64,

    /// effective thickness
    effective: f64,
}

#[derive(Debug, Parser)]
struct FindNormal {
    /// required effective thickness
    effective: f64,

    /// expected angle of incidence
    angle: f64,
}

fn main() {
    run(&Args::parse());
}

fn run(args: &Args) {
    match &args.command {
        Command::LineOfSight(los) => calculate_line_of_sight(los),
        Command::FindSlope(slope) => find_slope(slope),
        Command::FindNormal(normal) => find_normal(normal),
    }
}

/// Calculate the effective thickness of a plate at a given slope
fn calculate_line_of_sight(args: &LineOfSight) {
    let line_of_sight = args.normal / args.angle().cos();
    println!("{line_of_sight:.02}");
}

/// Calculate the slope of a plate based on normal and effective thickness
fn find_slope(args: &FindSlope) {
    let slope = (args.normal / args.effective).acos().to_degrees();
    println!("{slope:.02}");
}

/// Calculate the normal thickness of a plate presenting a given effective thickness at a given
/// expected angle of incidence
fn find_normal(args: &FindNormal) {
    let normal = args.effective * args.angle.to_radians().cos();
    println!("{normal:.02}");
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
