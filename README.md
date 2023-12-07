# nth_irreducible
This program uses a specially crafted algorithm to compute the nth binary irreducible polynomial. I was inspired to write this by https://projecteuler.net/problem=810 about xor-primes.

Some notes:
- A binary polynomial is a polynomial with coefficients equal to 0/1. When adding or multiplying these polynomials, any odd integer is interpreted as 1, and an even integer is interpreted as 0.
  They are more commonly known as polynomials over the Galois field of 2 elements. (see: https://en.wikipedia.org/wiki/GF(2), if you feel comfortable with such abstract mathematics).
- An irreducible binary polynomial is a binary polynomial with no factors other than 1 and itself, with respect to the multiplication mentioned.
  Every binary polynomial can be factored into irreducible polynomials in a unique way.
- The coefficients of binary polynomials can be stored as bits. In this format, addition corresponds to bitwise exclusive or (xor) and multiplication
   is similar to ordinary multiplication, but without carrying (xor multiplication).
- When written as a sequence of bits, binary polynomials can also be interpreted as an integer written in binary. For instance X^2 + 1 becomes 4 + 1 = 5.
  Using this interpretation, the binary irreducible polynomials can be ordered (alternatively, this is a lexicographic ordering starting from the most significant bit).
  As far as I know, this order does *not* have any algebraic significance.

Given a nonnegative integer n, the task is to compute the binary nth irreducible polynomial.
The 0th irreducible polynomial is X (2), the 1st is X + 1 (3), 2nd is X^2 + X + 1 (7), then come the 3rd X^3 + X + 1 (11) and 4th X^3 + X^2 + 1 (13) and it goes on and on (there are infinitely many).
One can draw all kinds of analogies from the prime numbers among ordinary integers to polynomials, for instance about their density.
However, there are well known formulas that allow easy computation of the number of irreducibles of each degree.
This is not enough to compute the nth irreducible polynomial, but it is enough to compute its degree.

The algorithm I designed consists of two parts.
1. In the first part, we compute the degree of the nth irreducible polynomial AND a fixed number k of its leading (most significant) bits. The algorithm for this is based on dynamic programming.
2. In the second part, we compute irreducibles in the search space specified by the obtained information in the first part until we have found the answer.

The time complexity is about O(2^(2D/3)) where D is the degree of the answer (disregarding some smaller log factors). The source code contains further explanations, but I have also written more about the algorithm in a post on project euler.

The program is written in rust. With the correct version of rust installed (written in 1.73.0), it can be run and tested by the appropriate cargo commands.
