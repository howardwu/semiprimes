use super::utils::*;

#[derive(Copy, Clone)]
pub struct BigInteger(pub [u64; 33]);

impl BigInteger {
    #[inline(always)]
    pub fn new(input: [u64; 33]) -> BigInteger {
        BigInteger(input)
    }

    #[inline(always)]
    pub fn is_odd(&self) -> bool { self.0[0] & 1 == 1 }

    #[inline(always)]
    pub fn is_even(&self) -> bool { !self.is_odd() }

    #[inline(always)]
    pub fn is_zero(&self) -> bool { self.0.iter().all(|&e| e == 0) }

    #[inline(always)]
    pub fn div2(&mut self) {
        let mut t = 0;
        for i in self.0.iter_mut().rev() {
            let t2 = *i << 63;
            *i >>= 1;
            *i |= t;
            t = t2;
        }
    }

    #[inline(always)]
    pub fn mul2(&mut self) {
        let mut last = 0;
        for i in self.0.iter_mut() {
            let tmp = *i >> 63;
            *i <<= 1;
            *i |= last;
            last = tmp;
        }
    }

    #[inline(always)]
    pub fn num_bits(&self) -> u32 {
        let mut ret = (33 as u32) * 64;
        for i in self.0.iter().rev() {
            let leading = i.leading_zeros();
            ret -= leading;
            if leading != 64 { break ; }
        }
        ret
    }

    #[inline(always)]
    pub fn add_nocarry(&mut self, other: &BigInteger) -> bool {
        let mut carry = 0;
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = adc(*a, *b, &mut carry);
        }
        carry != 0
    }

    #[inline(always)]
    pub fn sub_noborrow(&mut self, other: &BigInteger) -> bool {
        let mut borrow = 0;
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = sbb(*a, *b, &mut borrow);
        }
        borrow != 0
    }
}

impl ::rand::Rand for BigInteger {
    #[inline(always)]
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        let mut input = [0u64; 33];
        rng.fill(&mut input[..]);
        input[32] = 0u64;
        BigInteger(input)
    }
}

impl ::std::fmt::Debug for BigInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut result = "".to_string();
        for i in self.0.iter().rev() {
            result += format!("{:?}", i).as_str();
        }
        write!(f, "BigInteger({})", result)
    }
}

impl AsRef<[u64]> for BigInteger {
    #[inline(always)]
    fn as_ref(&self) -> &[u64] { &self.0 }
}

impl From<u64> for BigInteger {
    #[inline(always)]
    fn from(val: u64) -> BigInteger {
        let mut repr = BigInteger([0u64; 33]);
        repr.0[0] = val;
        repr
    }
}

impl Ord for BigInteger {
    #[inline(always)]
    fn cmp(&self, other: &BigInteger) -> ::std::cmp::Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a < b {
                return ::std::cmp::Ordering::Less
            } else if a > b {
                return ::std::cmp::Ordering::Greater
            }
        }
        ::std::cmp::Ordering::Equal
    }
}

impl PartialOrd for BigInteger {
    #[inline(always)]
    fn partial_cmp(&self, other: &BigInteger) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BigInteger {}

impl PartialEq for BigInteger {
    #[inline]
    fn eq(&self, other: &BigInteger) -> bool {
        match *other {
            BigInteger(ref other_data) => match *self {
                BigInteger(ref self_data) => (*self_data)[..] == (*other_data)[..],
            },
        }
    }
    #[inline]
    fn ne(&self, other: &BigInteger) -> bool {
        match *other {
            BigInteger(ref other_data) => match *self {
                BigInteger(ref self_data) => (*self_data)[..] != (*other_data)[..],
            },
        }
    }
}
