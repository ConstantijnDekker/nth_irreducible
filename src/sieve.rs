use super::polys::{Degree, Poly, self};

// Compute irreducibles of degree at most d using a sieve.
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

// Convert an index to the corresponding polynomial.
fn idx_to_poly(d: Degree, f: Poly, k: Degree, idx: usize) -> Poly {
    (1 << d) + ((idx as u64) << k) + f
}

// Convert a polynomial to its corresponding index.
fn poly_to_idx(d: Degree, k: Degree, g: Poly) -> usize {
    ((g ^ (1 << d)) >> k) as usize
}

// Compute all irreducibles of degree d, congruent to f modulo x^k.
// Take the happy path where 2k <= d.
// In that case the degree of a sieving prime w
pub fn get_irreds(d: Degree, f: Poly, k: Degree) -> Vec<Poly> {
    dbg!(d - k);
    let small_irreds = sieve_erat(d / 2);
    let mut is_irred: Vec<bool> = vec![true; 1 << (d - k)];

    for g in small_irreds {
        let r = d - polys::degree(g); // we must have r >= k.
        let h = (polys::xor_mult(polys::get_inverse(g, k), f) & ((1 << k) - 1)) + (1 << r);
        for i in 0..(1 << (r - k)) {
            is_irred[poly_to_idx(d, k, polys::xor_mult(h + (i << k), g))] = false;
            //is_irred[((xor_mult(h + (i << k), g) ^ (1 << d)) >> k) as usize] = false;
        }
    }

    collect_irreds(d, f, k, &is_irred)
}

//#[cfg_attr(target_arch = "x86_64", target_feature(enable = "bmi1"))]
fn collect_irreds(d: Degree, f: Poly, k: Degree, is_irred: &[bool]) -> Vec<Poly> {
    let mut irreducibles = Vec::new();
    let mut idx = 0;
    if is_irred[idx] {
        irreducibles.push(idx_to_poly(d, f, k, idx));
    }
    for i in 1u64..(1 << (d - k)) {
        let w = (i.trailing_zeros() + 1) as i64;
        let mask = ((1 << w) - 1) << ((d - k) - w);
        idx ^= mask;
        if is_irred[idx] {
            irreducibles.push(idx_to_poly(d, f, k, idx));
        }
    }
    irreducibles

    // Reversing bits is expensive, but we can perhaps iterate over them in reverse
    //(0u64..(1 << (d - k))).filter(|&i| is_irred[i as usize]).map(|i| (1 << d) + (i << k) + f).collect()
    //(0u64..(1 << (d - k))).map(|i| i.reverse_bits() >> (64 - d + k)).filter(|&i| is_irred[i as usize]).map(|i| (1 << d) + (i << k) + f).collect()
}
