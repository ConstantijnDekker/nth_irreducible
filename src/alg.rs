use super::count;
use super::polys::{self, Degree, Poly};
use super::sieve;

// Number of irreducibles of each degree. These are not hard to compute, but this is a well-known sequence.
const IRRED_OF_DEG: [i64; 64] = [
    0,
    2,
    1,
    2,
    3,
    6,
    9,
    18,
    30,
    56,
    99,
    186,
    335,
    630,
    1161,
    2182,
    4080,
    7710,
    14532,
    27594,
    52377,
    99858,
    190557,
    364722,
    698870,
    1342176,
    2580795,
    4971008,
    9586395,
    18512790,
    35790267,
    69273666,
    134215680,
    260300986,
    505286415,
    981706806,
    1908866960,
    3714566310,
    7233615333,
    14096302710,
    27487764474,
    53634713550,
    104715342801,
    204560302842,
    399822314775,
    781874934568,
    1529755125849,
    2994414645858,
    5864061663920,
    11488774559616,
    22517997465744,
    44152937520670,
    86607683851185,
    169947155749830,
    333599969907456,
    655069036708398,
    1286742745883790,
    2528336632900554,
    4969489234738635,
    9770521225481754,
    19215358392200893,
    37800705069076950,
    74382032520643617,
    146402730743693304,
];
// Compute the degree of the nth irreducible polynomial
fn nth_irreducible_degree(n: i64) -> Degree {
    let mut d = 0;
    let mut num_irred = 0;
    while num_irred <= n {
        d += 1;
        num_irred += IRRED_OF_DEG[d as usize]
    }
    d
}

// Compute the remainder modulo x^k, of the idx-th polynomial of degree deg.
fn get_remainder(deg: i64, idx: i64, k: i64) -> (Poly, i64) {
    let rem_to_irred = count::count_irreds_with_remainder(deg, k);
    let mut num_irred = 0;
    // Iterate in bit reverse order, because this
    // resembles how the remainders are ordered when they are rotated and
    // on the start (leading).
    for rev_rem in (1 << (k - 1))..(1 << k) {
        let rem = polys::reverse(rev_rem, k);
        let extra = rem_to_irred[(rem >> 1) as usize];
        if num_irred + extra > idx {
            return (rem as Poly, idx - num_irred);
        }
        num_irred += extra; // num_irred <= idx
    }
    (0, 0)
}

// Insert special case if n == 0
pub fn nth_irreducible(n: i64) -> Poly {
    let deg = nth_irreducible_degree(n);
    if deg <= 2 {
        return [0b10, 0b11, 0b111][n as usize];
    }
    let num_irred = (1..deg).map(|d| IRRED_OF_DEG[d as usize]).sum::<i64>();
    let idx = n - num_irred;
    let k = std::cmp::max(2, (deg + 2) / 3);
    let now = std::time::Instant::now();
    let (f, idx) = get_remainder(deg, idx, k);
    let irreds = sieve::get_irreds(deg, polys::reverse(f, k) << (deg + 1 - k), k);
    irreds[idx as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nth_irreducible_small() {
        assert_eq!(nth_irreducible(0), 0b10);
        assert_eq!(nth_irreducible(1), 0b11);
        assert_eq!(nth_irreducible(2), 0b111);
        assert_eq!(nth_irreducible(3), 0b1011);
        assert_eq!(nth_irreducible(4), 0b1101);
        assert_eq!(nth_irreducible(5), 0b10011);
        assert_eq!(nth_irreducible(6), 0b11001);
        assert_eq!(nth_irreducible(7), 0b11111);
    }

    #[test]
    fn test_nth_irreducible100() {
        assert_eq!(nth_irreducible(100), 0b1100010011);
    }
}
