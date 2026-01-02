mod data;
mod genome;
mod individual;
mod population;

use clap::Parser;
use data::TestSet;
use nalgebra::dvector;

const NUM_GENERATIONS: usize = 100;
const N_FITTEST_REPRODUCE: usize = 20;

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

    let mut pop = population::Population::new::<2, 1, 100>();

    let xor_test_inputs: TestSet = TestSet {
        inputs: vec![
            dvector![0.0, 0.0],
            dvector![0.0, 1.0],
            dvector![1.0, 0.0],
            dvector![1.0, 1.0],
        ],
        outputs: vec![dvector![0.0], dvector![1.0], dvector![1.0], dvector![0.0]],
    };

    for _generation in 0..NUM_GENERATIONS {
        pop = pop.reproduce(&xor_test_inputs, N_FITTEST_REPRODUCE);
    }
}
