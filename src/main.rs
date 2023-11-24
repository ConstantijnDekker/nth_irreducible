mod polys;
mod count;
mod sieve;
mod alg;

use self::polys::*;
use self::count::*;




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
            println!(
                "Found {} in {} seconds",
                polys::poly_to_string(f),
                now.elapsed().as_micros() as f64 / 1_000_000.
            );
        }
        _ => {
            println!("Usage: [PROGRAM_NAME] [n]");
        }
    }
}
