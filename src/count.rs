use super::polys::*;


// Mark the multiples of the form
// (degree d leftover) * an irreducible with degree r congruent to r modulo x^k.
fn mark(leftovers: &mut [Vec<i64>], d: i64, r: i64, f: Poly, k: i64) {
    for g in (1..(1 << k)).step_by(2) {
        let h = xor_mult(g, f) & ((1 << k) - 1); // last k bits of g times f
        leftovers[(d + r) as usize][(h >> 1) as usize] -= leftovers[d as usize][(g >> 1) as usize];
    }
}

// Remove multiples of irreducible of degree r congruent to f modulo x^k
// from the leftover list.
fn remove_multiples(leftovers: &mut [Vec<i64>], r: i64, f: Poly, k: i64) {
    let deg = leftovers.len() as i64 - 1;
    mark(leftovers, deg - r, r, f, k);
    // We don't have to do all degrees because the values they 
    // help compute are not necessary anymore for future computations.
    for d in (r..(deg - 2 * r)).rev() {
        mark(leftovers, d, r, f, k);
    }   
    // remove the multiple f itself.
    leftovers[r as usize][(f >> 1) as usize] -= 1;
}

// Return a vector containing at position f
// all irreducibles with remainder Xf + 1 of degree deg.
pub fn count_irreds_with_remainder(deg: i64, k: i64) -> Vec<i64> {
    let mut leftovers: Vec<Vec<i64>> = vec![vec![0; 1 << (k - 1)]; deg as usize + 1];

    // Initialise the irreducible candidates that we will remove.
    for g in (1..(1 << k)).step_by(2) {
        for d in 0..=deg {
            if d < k {
                if d == degree(g) {
                    leftovers[d as usize][(g >> 1) as usize] = 1;
                }
            } else if degree(g) <= d {
                leftovers[d as usize][(g >> 1) as usize] = 1 << (d - k);
            }
        }
    }

    // Remove multiples of each remaining prime in degree order
    for d in 1..=(deg / 2) {
        for g in (1..(1 << k)).step_by(2) {
            for _ in 0..leftovers[d as usize][(g >> 1) as usize] {
                remove_multiples(&mut leftovers, d, g, k);
            }
        }
    }

    leftovers[deg as usize].clone()
}