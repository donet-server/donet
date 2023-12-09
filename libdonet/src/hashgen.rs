// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

pub struct PrimeNumberGenerator {
    primes: Vec<u16>,
}

impl PrimeNumberGenerator {
    pub fn new() -> PrimeNumberGenerator {
        PrimeNumberGenerator { primes: vec![2_u16] }
    }
    /* Returns the nth prime number.  this[0] returns 2, this[1] returns 3;
     * successively larger values of n return larger prime numbers, up to the
     * largest prime number that can be represented in an int.
     */
    pub fn get_prime(&mut self, n: u16) -> u16 {
        assert_ne!(n >= 0, false);

        // Compute the prime numbers between the last-computed prime number and n.
        let mut candidate: u16 = self.primes.last().unwrap() + 1_u16;
        while self.primes.len() <= usize::from(n) {
            /* Is candidate prime?  It is not if any one of the already-found prime
             * numbers (up to its square root) divides it evenly.
             */
            let mut maybe_prime: bool = true;
            let mut j: usize = 0;
            while maybe_prime && self.primes.get(j).unwrap() * self.primes.get(j).unwrap() <= candidate {
                if (self.primes.get(j).unwrap() * (candidate / self.primes.get(j).unwrap())) == candidate {
                    // This one is not prime.
                    maybe_prime = false;
                }
                j += 1;
                assert_ne!(j < self.primes.len(), false);
            }
            if maybe_prime {
                self.primes.push(candidate);
            }
            candidate += 1;
        }
        *self.primes.get(usize::from(n)).unwrap()
    }
}
