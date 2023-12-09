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

pub static MAX_PRIME_NUMBERS: u16 = 1000;

pub struct PrimeNumberGenerator {
    primes: Vec<u16>,
}

pub struct PandaLegacyHashGenerator {
    hash: u32, // 32-bit hash
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

impl PandaLegacyHashGenerator {
    pub fn new() -> PandaLegacyHashGenerator {
        PandaLegacyHashGenerator {
            hash: 0_u32,
            index: 0_u16,
            primes: PrimeNumberGenerator::new(),
        }
    }
    // Adds another integer to the hash so far.
    pub fn add_int(&mut self, number: u32) {
        assert!(self.index < MAX_PRIME_NUMBERS);
        self.hash += u32::from(self.primes.get_prime(self.index)) * number;
        self.index = (self.index + 1) % MAX_PRIME_NUMBERS;
    }

    // Adds a blob to the hash, by breaking it down into a sequence of integers.
    pub fn add_blob(&mut self, blob: Vec<u8>) {
        self.add_int(blob.len().try_into().unwrap());
        for byte in blob.into_iter() {
            self.add_int(u32::from(byte));
        }
    }
    // Adds a string to the hash, by breaking it down into a sequence of integers.
    pub fn add_string(&mut self, string: String) {
        self.add_blob(string.into_bytes());
    }

    pub const fn get_hash(&self) -> u32 {
        self.hash & 0xffffffff
    }
}
