use crate::InternalKey;
#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSerialize};

/// Represent 256 bits
#[derive(Eq, PartialEq, Debug, Default, Hash, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
pub struct H256([u8; 32]);

const ZERO: H256 = H256([0u8; 32]);
const MAX_INDEX: u8 = 31;
const BYTE_SIZE: u8 = 8;

impl H256 {
    pub const fn zero() -> Self {
        ZERO
    }

    pub fn is_zero(&self) -> bool {
        self == &ZERO
    }

    #[inline]
    pub fn get_bit(&self, i: u8) -> bool {
        let byte_pos = MAX_INDEX - i / BYTE_SIZE;
        let bit_pos = i % BYTE_SIZE;
        let bit = self.0[byte_pos as usize] >> bit_pos & 1;
        bit != 0
    }

    #[inline]
    pub fn set_bit(&mut self, i: u8) {
        let byte_pos = MAX_INDEX - i / BYTE_SIZE;
        let bit_pos = i % BYTE_SIZE;
        self.0[byte_pos as usize] |= 1 << bit_pos as u8;
    }

    #[inline]
    pub fn clear_bit(&mut self, i: u8) {
        let byte_pos = MAX_INDEX - i / BYTE_SIZE;
        let bit_pos = i % BYTE_SIZE;
        self.0[byte_pos as usize] &= !((1 << bit_pos) as u8);
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    /// Treat H256 as a path in a tree
    /// fork height is the number of common bits(from heigher to lower: 255..=0)
    /// of two H256
    pub fn fork_height(&self, key: &H256) -> u8 {
        for h in (0..=core::u8::MAX).rev() {
            if self.get_bit(h) != key.get_bit(h) {
                return h;
            }
        }
        0
    }

    /// Treat H256 as a path in a tree
    /// return parent_path of self
    pub fn parent_path(&self, height: u8) -> Self {
        height
            .checked_add(1)
            .map(|i| self.copy_bits(i..))
            .unwrap_or_else(H256::zero)
    }

    /// Copy bits and return a new H256
    pub fn copy_bits(&self, range: impl core::ops::RangeBounds<u8>) -> Self {
        const MAX: usize = 256;
        const ARRAY_SIZE: usize = 32;
        const BYTE: usize = 8;
        use core::ops::Bound;

        let mut target = H256::zero();
        let start = match range.start_bound() {
            Bound::Included(&i) => i as usize,
            Bound::Excluded(&i) => panic!("do not allows excluded start: {}", i),
            Bound::Unbounded => 0,
        };

        let mut end = match range.end_bound() {
            Bound::Included(&i) => i.saturating_add(1) as usize,
            Bound::Excluded(&i) => i as usize,
            Bound::Unbounded => MAX,
        };

        if start >= MAX {
            return target;
        } else if end > MAX {
            end = MAX;
        }

        if end < start {
            panic!("end can't less than start: start {} end {}", start, end);
        }

        let end_byte = {
            let remain = if start % BYTE != 0 { 1 } else { 0 };
            ARRAY_SIZE - start / BYTE - remain
        };
        let start_byte = ARRAY_SIZE - end / BYTE;
        // copy bytes
        if start_byte < self.0.len() && start_byte <= end_byte {
            target.0[start_byte..end_byte].copy_from_slice(&self.0[start_byte..end_byte]);
        }

        // copy remain bits
        for i in (start..core::cmp::min((ARRAY_SIZE - end_byte) * BYTE, end))
            .chain(core::cmp::max((ARRAY_SIZE - start_byte) * BYTE, start)..end)
        {
            if self.get_bit(i as u8) {
                target.set_bit(i as u8)
            }
        }
        target
    }
}

impl From<[u8; 32]> for H256 {
    fn from(v: [u8; 32]) -> H256 {
        H256(v)
    }
}

impl From<H256> for [u8; 32] {
    fn from(v: H256) -> [u8; 32] {
        v.0
    }
}

/// A wrapper type for using a hash an internal key
#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize))]
pub struct Hash(InternalKey<32>);

impl core::ops::Deref for Hash {
    type Target = InternalKey<32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<H256> for Hash {
    fn from(hash: H256) -> Self {
        let hash: [u8; 32] = hash.into();
        Self(hash.into())
    }
}

impl From<[u8; 32]> for Hash {
    fn from(hash: [u8; 32]) -> Self {
        Self(hash.into())
    }
}
