mod alg;
mod count;
mod polys;
mod sieve;

use self::count::*;
use self::polys::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let now = std::time::Instant::now();
            let n: i64 = args[1]
                .replace('_', "")
                .parse()
                .expect("Invalid numeric literal format");
            let f = alg::nth_irreducible(n);
            let comp_time = now.elapsed().as_micros() as f64 / 1_000_000.;
            println!("nth-irreducible (string): {}.", poly_to_string(f));
            println!("nth-irreducible (numeric): {f}");
            println!("Computation time: {comp_time} seconds");
        }
        _ => {
            println!("Usage: [PROGRAM_NAME] [n]");
        }
    }
}
