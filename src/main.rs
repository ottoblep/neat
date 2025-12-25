pub mod population;

use clap::Parser;
use population::Genome;

use crate::population::Individual;

#[derive(Parser, Debug)]
#[clap(author = "", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from neat)!",
        args.name.unwrap_or("world".to_string())
    );

    let mut genom: Genome = Genome::new::<3, 2>();
    let mut individ: Individual = Individual::new(genom.clone());
    let mut res = individ.evaluate::<3, 2>(nalgebra::SVector::<f32, 3>::from_column_slice(&[
        1.0, 0.5, -1.0,
    ]));
}
