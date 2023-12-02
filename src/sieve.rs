/* This module contains the functions necessary for sieving.
 * This is the process of removing all of the multiples from a set
 * (in our case of irreducibles) from another set, which in our case
 * is either the set of polynomials of degree <= some bound
 * or it is the set of polynomials with certain leading bits
 * and length. */

use super::polys::{self, Degree, Poly};

// Amount of bits necessary to specify a bit in a 64-bit integer.
const WORD_LEN: i64 = 6;

// Compute odd irreducibles of degree at most d using a sieve.
fn sieve_erat(d: Degree) -> Vec<Poly> {
    let mut irreducibles: Vec<u64> = Vec::new();
    let mut is_irred: Vec<u64> = vec![0; 1 << std::cmp::max(d - WORD_LEN, 0)];

    for g in (3..(1 << (d + 1))).step_by(2) {
        if is_zero_bit((g >> 1) as usize, &is_irred) {
            irreducibles.push(g);
            let r = polys::degree(g);
            for h in (1..(1 << (d + 1 - r))).step_by(2) {
                let multiple = polys::xor_mult(h, g);
                set_bit((multiple >> 1) as usize, &mut is_irred);
            }
        }
    }

    irreducibles
}

// Compute the idx-th irreducible of degree d starting with first k bits equal to f.
// parition the number up like this
// [f (k bits)][blck_idx] (d - k - d/2) bits][d/2 bits]1
// And for each [f][blck_idx], call find_with_sieve.
pub fn find_irreducible(f: Poly, k: Degree, idx: i64) -> Option<Poly> {
    let d = polys::degree(f);
    let small_irreds = sieve_erat(d / 2);
    let sieve_len = d / 2;
    let mut total_irred = 0;
    for blck_idx in 0..(1 << (d - k - sieve_len)) {
        let (g, num_irred) = find_with_sieve(
            f + (blck_idx << (sieve_len + 1)),
            d - sieve_len,
            &small_irreds,
            idx - total_irred,
        );
        if g.is_some() {
            return g;
        }
        total_irred += num_irred;
    }
    None
}

// Convert a polynomial to its corresponding index according to the
// mapping used by the find_with_sieve function.
// g == 1[k - bits][d - k bits]1
// We are interested in the [d - k bits].
fn poly_to_idx(d: Degree, k: Degree, g: Poly) -> usize {
    (polys::mod_red(g, d + 1 - k) >> 1) as usize
}

fn split_idx(bit_idx: usize) -> (usize, usize) {
    (bit_idx >> WORD_LEN, bit_idx & ((1 << WORD_LEN) - 1))
}

fn set_bit(bit_idx: usize, is_irred: &mut [u64]) {
    let (idx, bit_offset) = split_idx(bit_idx);
    is_irred[idx] |= 1 << bit_offset;
}

fn is_zero_bit(bit_idx: usize, is_irred: &[u64]) -> bool {
    let (idx, bit_offset) = split_idx(bit_idx);
    is_irred[idx] & (1 << bit_offset) == 0
}

// Compute the idx-th irreducible of degree d starting with f.
// OR the number of irreducibles of degree d starting with f.
//
// Assume k <= (d + 1) / 2 (floored).
// Highest degree of an irreducible is d/2 (floored)
// If d is odd, then d / 2 (floored) = (d - 1) / 2 and
// (d + 1) / 2 = d - (d - 1) / 2 >= k.
// If d is even, (d / 2) floored == d / 2.
// Then d - d / 2 == d / 2 == (d + 1) / 2 (floored) >= k.
// This shows every irreducible has at least one multiple occuring in the block.
fn find_with_sieve(
    f: Poly,
    k: Degree,
    small_irreds: &[Poly],
    idx: i64,
) -> (Option<Poly>, i64) {
    let d = polys::degree(f);
    let mut is_irred: Vec<u64> = vec![0; 1 << std::cmp::max(0, d - k - WORD_LEN)];

    for &g in small_irreds {
        let r = d - polys::degree(g); // we must have r >= k.
        let h = polys::comp_multiplier(f, g, k);
        for i in 0..(1 << (r - k)) {
            let multiple = polys::xor_mult(h + (i << 1), g);
            let bit_idx = poly_to_idx(d, k, multiple);
            set_bit(bit_idx, &mut is_irred);
        }
    }

    let mut num_irred = 0;
    for i in 0u64..(1 << (d - k)) {
        if is_zero_bit(i as usize, &is_irred) {
            num_irred += 1;
            if num_irred == idx + 1 {
                return (Some(f + (i << 1) + 1), num_irred);
            }
        }
    }
    (None, num_irred)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_idx() {
        assert_eq!(split_idx(0b1111001), (0b1, 0b111001));
    }

    #[test]
    fn test_sieve_erat() {
        assert_eq!(sieve_erat(3), vec![0b11, 0b111, 0b1011, 0b1101]);
    }

    #[test]
    fn test_find_irreducible() {
        assert_eq!(find_irreducible(0b1100000, 2, 1), Some(0b1100111));
    }

    #[test]
    fn test_find_with_sieve_success() {
        let small_irreds = sieve_erat(3);
        assert_eq!(find_with_sieve(0b1110000, 3, &small_irreds, 1), (Some(0b1110101), 2));
    }

    #[test]
    fn test_find_with_sieve_failure() {
        let small_irreds = sieve_erat(3);
        assert_eq!(find_with_sieve(0b1110000, 3, &small_irreds, 2), (None, 2));
    }
}