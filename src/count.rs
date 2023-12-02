/* This module counts the number of irreducibles
 * with a certain (odd) remainder using dynamic programming. */
use super::polys::{self, Degree, Poly};

// Initialise leftovers, where leftovers[d][g >> 1] is the number of polynomials
// of degree d with remainder g modulo X^k (assuming g == 1 mod X).
fn initialise(deg: Degree, k: Degree) -> Vec<Vec<i64>> {
    let mut leftovers: Vec<Vec<i64>> = vec![vec![0; 1 << (k - 1)]; deg as usize + 1];
    // Polynomials of degree less than equal to k only have remainders of their degree
    for d in 0..k {
        for g in ((1 << d)..(1 << (d + 1))).step_by(2) {
            leftovers[d as usize][(g >> 1) as usize] = 1;
        }
    }
    for d in k..=deg {
        for g in (1..(1 << k)).step_by(2) {
            leftovers[d as usize][(g >> 1) as usize] = 1 << (d - k);
        }
    }

    leftovers
}

// Return leftovers[deg], where leftovers[deg][g >> 1]
// is the number of irreducibles of degree deg with remainder g modulo X^k
// for all g == 1 mod X.
pub fn count_irreds_with_remainder(deg: Degree, k: Degree) -> Vec<i64> {
    let mut leftovers = initialise(deg, k);

    // Remove multiples of each remaining prime in degree order
    for d in 1..=(deg / 3) {
        // At this point, leftovers[d] counts irreducibles of degree d.
        for g in (1..(1 << k)).step_by(2) {
            // For each irreducible congruent to g, we remove
            // all of its multiples that haven't been removed already.
            for _ in 0..leftovers[d as usize][(g >> 1) as usize] {
                remove_multiples(&mut leftovers, d, g, k);
            }
        }
    }

    // For irreducibles of larger degree, the only
    // multiples to remove are twin primes.
    for d in (deg / 3 + 1)..=(deg / 2) {
        remove_semis(&mut leftovers, d, deg - d, k);
    }

    leftovers[deg as usize].clone()
}

// Remove semi-irreducibles that are the product of degree a and degree b irreducible.
// Assume leftovers[a] and leftovers[b] contains the irreducible counts.
fn remove_semis(leftovers: &mut [Vec<i64>], a: Degree, b: Degree, k: Degree) {
    // If a and b are the same, the calculation is more complicated.
    if a == b {
        for g in (1..(1 << k)).step_by(2) {
            let prod_rem = polys::mod_red(polys::xor_mult(g, g), k);
            let num_twins = (leftovers[a as usize][(g >> 1) as usize]
                * (leftovers[a as usize][(g >> 1) as usize] + 1))
                / 2;
            leftovers[(a + b) as usize][(prod_rem >> 1) as usize] -= num_twins;
            for h in ((g + 2)..(1 << k)).step_by(2) {
                let prod_rem = polys::mod_red(polys::xor_mult(g, h), k);
                let num_twins = leftovers[a as usize][(g >> 1) as usize]
                    * leftovers[b as usize][(h >> 1) as usize];
                leftovers[(a + b) as usize][(prod_rem >> 1) as usize] -= num_twins;
            }
        }
    } else {
        for g in (1..(1 << k)).step_by(2) {
            for h in (1..(1 << k)).step_by(2) {
                let prod_rem = polys::mod_red(polys::xor_mult(g, h), k);
                let num_twins = leftovers[a as usize][(g >> 1) as usize]
                    * leftovers[b as usize][(h >> 1) as usize];
                leftovers[(a + b) as usize][(prod_rem >> 1) as usize] -= num_twins;
            }
        }
    }
}

// Subtract the multiples of the form
// g * (an irreducible with degree r congruent to f modulo X^k)
// where g is a polynomial that is not a multiple of any of the previous polynomials.
fn mark_multiples_of_deg(leftovers: &mut [Vec<i64>], d: Degree, r: Degree, f: Poly, k: Degree) {
    for g in (1..(1 << k)).step_by(2) {
        let h = polys::mod_red(polys::xor_mult(g, f), k); // last k bits of g times f
        leftovers[(d + r) as usize][(h >> 1) as usize] -= leftovers[d as usize][(g >> 1) as usize];
    }
}

// Remove multiples of irreducible of degree r congruent to f modulo X^k
// Here 'remove' means subtract from the total.
fn remove_multiples(leftovers: &mut [Vec<i64>], r: Degree, f: Poly, k: Degree) {
    let deg = leftovers.len() as i64 - 1;
    mark_multiples_of_deg(leftovers, deg - r, r, f, k);
    // We don't have to do all degrees because the values they
    // help compute are not necessary anymore for future computations.
    // Equal sign is only necessary because multiple irreducibles of degree r
    // can appear.
    for d in (r..=(deg - 2 * r)).rev() {
        mark_multiples_of_deg(leftovers, d, r, f, k);
    }
    // Remove the multiple f itself.
    leftovers[r as usize][(f >> 1) as usize] -= 1;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_initialise() {
        let leftovers = initialise(6, 3);
        assert_eq!(
            leftovers,
            vec![
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 1],
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![4, 4, 4, 4],
                vec![8, 8, 8, 8]
            ]
        );
    }
}
