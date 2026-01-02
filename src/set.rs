/// Set that holds usize values from 0 to 63 in the bits of a `u64` value. Overflows of the inserted
/// index may panic.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitSet {
    bits: u64,
}

impl BitSet {
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn insert(&mut self, index: usize) {
        self.bits |= 1 << index;
    }

    pub fn contains(&self, index: usize) -> bool {
        self.bits & (1 << index) != 0
    }

    pub fn is_subset_of(self, other: Self) -> bool {
        (self & other) == self
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + use<'_> {
        (0..64usize)
            .into_iter()
            .filter(|n| self.bits & (1 << *n) != 0)
    }

    pub fn into_inner(self) -> u64 {
        self.bits
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut res = BitSet::default();
        for index in iter {
            res.insert(index);
        }
        res
    }
}

// Implement set union
impl std::ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl std::ops::BitOr for BitSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self {
        self |= rhs;
        self
    }
}

// Implement set intersection
impl std::ops::BitAndAssign for BitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
    }
}

impl std::ops::BitAnd for BitSet {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}

// Implement set subtraction
impl std::ops::SubAssign for BitSet {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits &= !rhs.bits;
    }
}

impl std::ops::Sub for BitSet {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}
