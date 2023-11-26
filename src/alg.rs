use super::polys::{self, Poly, Degree};
use super::sieve;
use super::count;

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
    //let mut rems: Vec<Poly> = (1..(1 << k)).step_by(2).collect::<Vec<_>>();
    //rems.sort_by_key(|&rem| rem.reverse_bits());
    let mut num_irred = 0;
    let mut rem: usize = 0;
    let extra = rem_to_irred[rem];
    if num_irred + extra > idx as i64 {
        return ((2 * rem + 1) as Poly, rem as i64 - num_irred);
    }
    num_irred += extra;
    for i in 1u64..(1 << (k - 1)) {
        let w = (i.trailing_zeros() + 1) as i64;
        let mask = ((1 << w) - 1) << (k - 1 - w);
        rem ^= mask;
        let extra = rem_to_irred[rem];
        if num_irred + extra > idx {
            return ((2 * rem + 1) as Poly, idx - num_irred);
        }
        num_irred += extra;
    }

    /*
    for rem in rems {
        let extra = rem_to_irred[(rem >> 1) as usize];
        if num_irred + extra > idx {
            return (rem as Poly, idx - num_irred);
        }
        num_irred += extra; // num_irred <= idx
    }
    */
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
    let t1 = now.elapsed().as_micros();
    let irreds = sieve::get_irreds(deg, f, k);
    let t2 = now.elapsed().as_micros();
    dbg!(t1, t2 - t1);
    // sort_by?
    polys::reverse(irreds[idx as usize])
    //irreds[idx as usize].reverse_bits() >> (63 - deg)
}
