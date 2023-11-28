// Binary polynomials are polynomials over the Galois field with 2 elements.
// They form a ring analogous to the integers
// In fact, when evaluating every binary polynomial at 2, we obtain
// a ring isomorphism to the nonnegative integers with operations
// carry-less add or xor (addition)
// carry-less multiplication (multiplication).
// We call the resulting integer the numeric value of the polynomial.
//
// The ring of binary polynomials is a Unique Factorization Domain.
// and this program computes certain irreducibles (or primes) in this ring.
//
// This program takes an argument n, and
// computes nth irreducible binary polynomial
// when ordered in lexicographic order with the higher
// order terms being most significant.

// For example the first binary irreducible polynomials in this order are
// 0 : X
// 1 : X + 1
// 2 : X^2 + X + 1
// 3 : X^3 + X + 1
// 4 : X^3 + X^2 + 1
// 5 : X^4 + X + 1
// etc
// 
// This is the same ordering borrowed
// from the aforementioned isomorphism to the (carry-less) integers.
// As far as I know, this order does not have any algebraic significance,
// but it seems as good as any and it allows for efficient and insightful algorithmic
// ideas.

mod alg;
mod count;
mod polys;
mod sieve;

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
            println!("nth-irreducible (string): {}.", polys::poly_to_string(f));
            println!("nth-irreducible (numeric): {f}");
            println!("Computation time: {comp_time} seconds");
        }
        _ => {
            println!("Usage: [PROGRAM_NAME] [n]");
        }
    }
}
