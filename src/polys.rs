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

pub fn reverse(f: Poly) -> Poly {
    f.reverse_bits() >> (63 - degree(f))
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


// Compute inverse of f modulo x^k.
pub fn get_inverse(f: Poly, k: i64) -> Poly {
    let mut r = f;
    let mut g = 1;
    for i in 1..k {
        if r & (1 << i) != 0 {
            r ^= f << i;
            g |= 1 << i;
        }
    }
    g
}