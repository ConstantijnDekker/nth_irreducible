use super::polys::*;

// Compute irreducibles of degree at most d using a sieve.
fn sieve_erat(d: Degree) -> Vec<Poly> {
    let mut irreducibles: Vec<u64> = Vec::new();
    let mut is_irred: Vec<bool> = vec![true; 1 << (d + 1)];

    for g in (3..(1 << (d + 1))).step_by(2) {
        if is_irred[g as usize] {
            irreducibles.push(g);
            let r = degree(g);
            for h in (1..(1 << (d + 1 - r))).step_by(2) {
                is_irred[xor_mult(h, g) as usize] = false;
            }
        }
    }

    irreducibles
}


// Compute all irreducibles of degree d, congruent to f modulo x^k.
// Take the happy path where 2k <= d.
// In that case the degree of a sieving prime w
pub fn get_irreds(d: i64, f: Poly, k: Degree) -> Vec<Poly> {
    let small_irreds = sieve_erat(d / 2);
    let mut is_irred: Vec<bool> = vec![true; 1 << (d - k)];

    for g in small_irreds {
        let r = d - degree(g); // we must have r >= k.
        let mut h = xor_mult(get_inverse(g, k), f) & ((1 << k) - 1);
        h += 1 << r;
        for i in 0..(1 << (r - k)) {
            is_irred[((xor_mult(h + (i << k), g) ^ (1 << d)) >> k) as usize] = false;
        }
    }

    (0..(1 << (d - k))).filter(|&i| is_irred[i as usize]).map(|i| (1 << d) + (i << k) + f).collect()
}