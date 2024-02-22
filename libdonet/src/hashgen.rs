// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.
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

use crate::globals::{DCFileHash, MAX_PRIME_NUMBERS};

pub struct PrimeNumberGenerator {
    primes: Vec<u16>,
}

pub struct DCHashGenerator {
    hash: i32,
    index: u16,
    primes: PrimeNumberGenerator,
}

impl PrimeNumberGenerator {
    pub fn new() -> PrimeNumberGenerator {
        PrimeNumberGenerator { primes: vec![2_u16] }
    }
    /* Returns the nth prime number. this[0] returns 2, this[1] returns 3;
     * successively larger values of n return larger prime numbers, up to the
     * largest prime number that can be represented in an int.
     */
    pub fn get_prime(&mut self, n: u16) -> u16 {
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
                assert!(j < self.primes.len());
            }
            if maybe_prime {
                self.primes.push(candidate);
            }
            candidate += 1;
        }
        *self.primes.get(usize::from(n)).unwrap()
    }
}

impl Default for DCHashGenerator {
    fn default() -> Self {
        Self {
            hash: 0_i32,
            index: 0_u16,
            primes: PrimeNumberGenerator::new(),
        }
    }
}

impl DCHashGenerator {
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds another integer to the hash so far.
    pub fn add_int(&mut self, number: i32) {
        assert!(self.index < MAX_PRIME_NUMBERS);
        self.hash += i32::from(self.primes.get_prime(self.index)) * number;
        self.index = (self.index + 1) % MAX_PRIME_NUMBERS;
    }

    /// Adds a blob to the hash, by breaking it down into a sequence of integers.
    pub fn add_blob(&mut self, blob: Vec<u8>) {
        self.add_int(blob.len().try_into().unwrap());
        for byte in blob.into_iter() {
            self.add_int(i32::from(byte));
        }
    }
    /// Adds a string to the hash, by breaking it down into a sequence of integers.
    pub fn add_string(&mut self, string: String) {
        self.add_blob(string.into_bytes());
    }

    pub const fn get_hash(&self) -> DCFileHash {
        self.hash as u32
    }
}

#[cfg(test)]
mod unit_testing {
    use super::PrimeNumberGenerator;

    #[test]
    fn prime_number_generator_integrity() {
        let mut png: PrimeNumberGenerator = PrimeNumberGenerator::new();

        let prime_numbers: Vec<u16> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
            101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
            197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
            311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
            431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
            557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
            661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
            809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
        ];

        for (i, target_prime) in prime_numbers.into_iter().enumerate() {
            assert_eq!(target_prime, png.get_prime(i.try_into().unwrap()));
        }
    }
}
