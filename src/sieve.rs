/* This module contains the functions necessary for sieving.
 * This is the process of removing all of the multiples from a set
 * (in our case of irreducibles) from another set, which in our case
 * is either the set of polynomials of degree <= some bound
 * or it is the set of polynomials with certain leading bits
 * and length. */

use super::polys::{self, Degree, Poly};

// Compute odd irreducibles of degree at most d using a sieve.
fn sieve_erat(d: Degree) -> Vec<Poly> {
    let mut irreducibles: Vec<u64> = Vec::new();
    let mut is_irred: Vec<bool> = vec![true; 1 << d];

    for g in (3..(1 << (d + 1))).step_by(2) {
        if is_irred[(g >> 1) as usize] {
            irreducibles.push(g);
            let r = polys::degree(g);
            for h in (1..(1 << (d + 1 - r))).step_by(2) {
                is_irred[(polys::xor_mult(h, g) >> 1) as usize] = false;
            }
        }
    }

    irreducibles
}

// Compute the idx-th irreducible of degree d starting with f.
pub fn get_irreds(d: Degree, f: Poly, k: Degree, idx: i64) -> Option<Poly> {
    let small_irreds = sieve_erat(d / 2);
    let sieve_len = d / 2;
    let mut total_irred = 0;
    for i in 0..(1 << (d - k - sieve_len)) {
        let (f, num_irred) = sieve_block(
            d,
            f + (i << (sieve_len + 1)),
            d - sieve_len,
            &small_irreds,
            idx - total_irred,
        );
        if f.is_some() {
            return f;
        }
        total_irred += num_irred;
    }
    None
}

// Convert a polynomial to its corresponding index according to the
// mapping used by the sieve_block function.
// g == 1[k - bits][d - k bits]1
// We are interested in the [d - k bits].
fn poly_to_idx(d: Degree, k: Degree, g: Poly) -> usize {
    let mask = (1 << (d + 1 - k)) - 1;
    ((g & mask) >> 1) as usize
}

// Compute the idx-th irreducible of degree d starting with f.
// OR the number of irreducibles of degree d starting with f.
// Assume 2k <= d.
fn sieve_block(
    d: Degree,
    f: Poly,
    k: Degree,
    small_irreds: &[Poly],
    idx: i64,
) -> (Option<Poly>, i64) {
    let mut is_irred: Vec<bool> = vec![true; 1 << (d - k)];

    for &g in small_irreds {
        let r = d - polys::degree(g); // we must have r >= k.
        let h = polys::comp_multiplier(f, g, k);
        for i in 0..(1 << (r - k)) {
            is_irred[poly_to_idx(d, k, polys::xor_mult(h + (i << 1), g))] = false;
        }
    }

    let mut num_irred = 0;
    for i in 0u64..(1 << (d - k)) {
        if is_irred[i as usize] {
            num_irred += 1;
            if num_irred == idx + 1 {
                return (Some(f + (i << 1) + 1), num_irred);
            }
        }
    }
    (None, num_irred)
}
