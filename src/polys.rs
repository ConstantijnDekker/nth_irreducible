// This module contains types and functions for working with binary polynomials
// that are used throughout the program.
pub type Poly = u64;
pub type Degree = i64;

/* Convert polynomial to string. */
pub fn poly_to_string(f: Poly) -> String {
    if f == 0 {
        return "0".to_string();
    }
    let mut s: String = String::new();
    let mut first = true;
    for i in (0..64).rev() {
        if (f & (1 << i)) != 0 {
            if !first {
                s.push_str(" + ");
            }
            first = false;
            if i == 0 {
                s.push('1');
            } else if i == 1 {
                s.push('x');
            } else {
                s.push_str("x^");
                s.push_str(&i.to_string());
            }
        }
    }
    s
}

/* Find the degree of a polynomial. */
pub fn degree(f: Poly) -> Degree {
    (63 - f.leading_zeros()) as Degree
}

/* Reverse last k bits of a polynomial (and zero out others). */
pub fn reverse(f: Poly, k: Degree) -> Poly {
    f.reverse_bits() >> (64 - k)
}

/* Xor multiply with native assembly instruction. */
pub fn xor_mult(a: Poly, b: Poly) -> Poly {
    let mut a = a;
    unsafe {
        std::arch::asm!("pclmullqlqdq {xmm1}, {xmm2}",
             xmm1 = inout(xmm_reg) a,
             xmm2 = in(xmm_reg) b,
        );
    }
    a
}

// Compute odd h such that xor_mult(g, h) is odd and has
// the leading k bits equal to f.
pub fn comp_multiplier(f: Poly, g: Poly, k: Degree) -> Poly {
    let d = degree(f);
    let r = d - degree(g); // assume r >= k
    let mut res = f ^ g;
    let mut h = 1;
    for i in (1..=k).rev() {
        if res & (1 << (i + d - k)) != 0 {
            res ^= g << (i + r - k);
            h |= 1 << (i + r - k);
        }
    }
    h
}
